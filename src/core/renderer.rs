use nalgebra_glm as glm;
use super::{color::Color, framebuffer::Framebuffer, scene::Scene, ray::Ray, intersect::Intersect};
use super::geometry::RayIntersect;
use super::camera::OrbitCamera;
use rayon::prelude::*;

// Profundidad máxima de rayos secundarios
const MAX_DEPTH: u32 = 3;
// Pequeño sesgo para evitar acne de auto-intersección
const RAY_BIAS: f32 = 1e-4;

pub struct Renderer { }
impl Renderer {
    pub fn new() -> Self { Self {} }

    pub fn render_frame(&self, scene: &Scene, fb: &mut Framebuffer, cam: &OrbitCamera) {
        let width  = fb.width as f32;
        let height = fb.height as f32;
        let aspect = width / height;
    
        let cam_origin = cam.eye();
        let (right, up, forward) = cam.basis();
    
        // Precompute NDC for cols/rows 
        let w = fb.width as usize;
        let h = fb.height as usize;
    
        let mut sxs = Vec::with_capacity(w);
        for x in 0..w {
            // NDC x in [-1,1] with aspect
            let sx = (2.0 * x as f32) / width - 1.0;
            sxs.push(sx * aspect);
        }
        let mut sys = Vec::with_capacity(h);
        for y in 0..h {
            // NDC y in [-1,1]
            let sy = 1.0 - (2.0 * y as f32) / height;
            sys.push(sy);
        }
    
        // Buffer temporal para resultados por pixel (se llena en paralelo)
        let mut scratch: Vec<Color> = vec![Color::new(0,0,0); w * h];
    
        // Datos inmutables que capturamos en el closure (Send + Sync)
        let r = right;   // copia por valor (Vec3)
        let u = up;
        let f = forward;
        let cam_o = cam_origin;
    
        scratch
            .par_iter_mut()               // iteración paralela
            .enumerate()
            .for_each(|(idx, pix)| {
                let y = idx / w;
                let x = idx % w;
    
                let dir_world = glm::normalize(&(sxs[x] * r + sys[y] * u + f));
                let ray = Ray::new(cam_o, dir_world);
                let color = self.trace(&ray, scene, 0);
                *pix = color;
            });
    
        // Blit secuencial al framebuffer (barato vs. todo el cómputo anterior)
        for y in 0..h {
            let row = &scratch[y * w .. (y + 1) * w];
            for x in 0..w {
                fb.set_pixel(x as u32, y as u32, row[x]);
            }
        }
    }

    fn trace(&self, ray: &Ray, scene: &Scene, depth: u32) -> Color {
        if depth >= MAX_DEPTH {
            if let Some(sb) = &scene.skybox {
                return sb.sample(ray.dir);
            }
            return Color::new(135, 206, 235); // fallback cielo plano
        }        

        // --- Closest hit ---
        let mut closest = Intersect::miss();
        let mut zbuffer = f32::INFINITY;

        // Spheres
        for s in &scene.spheres {
            let hit = s.ray_intersect(&ray.origin, &ray.dir);
            if hit.is_intersecting && hit.distance < zbuffer {
                zbuffer = hit.distance;
                closest = hit;
            }
        }
        // AABBs (versión estándar)
        for b in &scene.aabbs {
            let hit = b.ray_intersect(&ray.origin, &ray.dir);
            if hit.is_intersecting && hit.distance < zbuffer {
                zbuffer = hit.distance;
                closest = hit;
            }
        }

        if !closest.is_intersecting {
            if let Some(sb) = &scene.skybox {
                return sb.sample(ray.dir);
            }
            return Color::new(135, 206, 235);
        }

        // --- Shading local (Phong básico) ---
        let local = self.shade_local(scene, &closest, ray.origin);

        // --- Reflexión / Transparencia ---
        let kr = closest.material.reflectivity.clamp(0.0, 1.0);
        let kt = closest.material.transparency.clamp(0.0, 1.0);
        let ior = closest.material.ior.max(1e-3);

        if kr == 0.0 && kt == 0.0 {
            return local;
        }

        let n = closest.normal; // normal saliente
        let mut refl_col = Color::new(0,0,0);
        let mut refr_col = Color::new(0,0,0);

        // Reflexión
        if kr > 0.0 {
            let refl_dir = reflect(ray.dir, n);
            let refl_origin = offset_origin(closest.point, n, refl_dir);
            let refl_ray = Ray::new(refl_origin, refl_dir);
            refl_col = self.trace(&refl_ray, scene, depth + 1);
        }

        // Refracción (Snell + TIR)
        let mut fresnel = 0.0;
        if kt > 0.0 {
            if let Some((refr_dir, fres)) = refract_with_fresnel(ray.dir, n, ior) {
                fresnel = fres; // proporción reflejada física
                let refr_origin = offset_origin(closest.point, -n, refr_dir);
                let refr_ray = Ray::new(refr_origin, refr_dir);
                refr_col = self.trace(&refr_ray, scene, depth + 1);
            } else {
                // TIR: todo se refleja
                fresnel = 1.0;
            }
        }

        // --- Mezcla de capas (dielectric-friendly) ---
        let kr = closest.material.reflectivity.clamp(0.0, 1.0);
        let kt = closest.material.transparency.clamp(0.0, 1.0);
        let ior = closest.material.ior.max(1e-3);

        // ¿Es dieléctrico transparente? (agua/vidrio)
        let is_dielectric = kt > 0.5 && ior > 1.001;

        // Piso suave de reflectividad para que el reflejo no desaparezca de frente
        let refl_floor = 0.20 * kr;

        let (mut w_local, mut w_refl, mut w_refr) = if kt > 0.0 {
            let w_refl_f = (refl_floor + (1.0 - refl_floor) * fresnel).clamp(0.0, 1.0) * kr;
            let w_refr_f = (1.0 - w_refl_f) * kt;
            let w_local_f = if is_dielectric { 0.0 } else { (1.0 - w_refl_f - w_refr_f).max(0.0) };
            (w_local_f, w_refl_f, w_refr_f)
        } else {
            ((1.0 - kr).max(0.0), kr, 0.0)
        };

        // Normaliza por seguridad
        let sum = (w_local + w_refl + w_refr).max(1e-6);
        w_local /= sum; w_refl /= sum; w_refr /= sum;

        // Composición final
        return mix3(local, refl_col, refr_col, w_local, w_refl, w_refr);
    }

    fn shade_local(&self, scene: &Scene, hit: &Intersect, cam_origin: glm::Vec3) -> Color {
        // Albedo (textura o color sólido)
        let albedo = hit.material.sample_albedo(hit.uv, hit.face).to_vec3();
        let mut result = albedo * 0.18; // ambiente
        let view_dir = glm::normalize(&(cam_origin - hit.point));

        for light in &scene.lights {
            let ldir = glm::normalize(&(light.position - hit.point));
            let n = hit.normal; // ya normalizada

            // Visibilidad (0 en sombra, 1 visible)
            let vis = shadow_visibility(scene, hit.point, n, light.position);
            if vis == 0.0 { continue; }

            // Difuso (Lambert)
            let ndotl = glm::dot(&n, &ldir);
            if ndotl <= 0.0 {
                // Si no hay difuso, tampoco hay especular Phong en este modelo
                continue;
            }
            let diff = ndotl;

            let light_col = light.color.to_vec3() * light.intensity;
            let mut add = albedo.component_mul(&light_col) * diff;

            // Especular (Phong) sólo si el material lo soporta
            if hit.material.specular > 0.0 {
                let r = reflect(-ldir, n);
                let specdot = glm::dot(&r, &view_dir).max(0.0);
                if specdot > 0.0 {
                    let spec = specdot.powf(hit.material.shininess.max(1.0));
                    add += light_col * (hit.material.specular * spec);
                }
            }

            // Aplica visibilidad
            result += vis * add;
        }

        Color::from_vec3(&result)
    }
}

#[inline]
fn reflect(i: glm::Vec3, n: glm::Vec3) -> glm::Vec3 {
    glm::normalize(&(i - 2.0 * glm::dot(&i, &n) * n))
}

/// Devuelve (dir_refractada, fresnel_reflectance) o None si hay TIR.
/// `ior` es n2/n1 (índice absoluto del material; tomamos n1=aire=1.0).
fn refract_with_fresnel(i: glm::Vec3, n: glm::Vec3, ior: f32) -> Option<(glm::Vec3, f32)> {
    // Normalizamos por seguridad
    let i = glm::normalize(&i);
    let mut n = glm::normalize(&n);
    let mut etai = 1.0;
    let mut etat = ior;
    let mut cosi = glm::dot(&i, &n).clamp(-1.0, 1.0);
    // Si cosi > 0, rayo viene desde dentro del material
    if cosi > 0.0 {
        core::mem::swap(&mut etai, &mut etat);
        n = -n;
    } else {
        cosi = -cosi;
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        // TIR
        return None;
    }
    let t = glm::normalize(&(eta * i + (eta * cosi - k.sqrt()) * n));

    // Fresnel (Schlick)
    let r0 = ((etat - etai) / (etat + etai)).powi(2);
    let fresnel = r0 + (1.0 - r0) * (1.0 - cosi).powi(5);

    Some((t, fresnel))
}

fn shadow_visibility(scene: &Scene, p: glm::Vec3, n: glm::Vec3, light_pos: glm::Vec3) -> f32 {
    let to_light = light_pos - p;
    let dist = glm::length(&to_light);
    if dist <= 0.0 { return 1.0; }

    let ldir = to_light / dist;

    let origin = offset_origin(p, n, ldir);
    let tmax = dist - RAY_BIAS;

    // Acumulamos visibilidad multiplicando transparencias de los bloqueadores
    // (1.0 = luz totalmente visible, 0.0 = completamente en sombra)
    let mut vis = 1.0_f32;

    // Spheres
    for s in &scene.spheres {
        let h = s.ray_intersect(&origin, &ldir);
        if h.is_intersecting && h.distance < tmax {
            let t = h.material.transparency.clamp(0.0, 1.0);
            if t <= 1e-3 { return 0.0; }     // bloqueador opaco: sombra dura
            vis *= t;                         // semitransparente: atenúa la luz
            if vis < 0.02 {
                return 0.0;
            }
        }
    }
    // AABBs
    for b in &scene.aabbs {
        let h = b.ray_intersect(&origin, &ldir);
        if h.is_intersecting && h.distance < tmax {
            let t = h.material.transparency.clamp(0.0, 1.0);
            if t <= 1e-3 { return 0.0; }
            vis *= t;
            if vis < 0.02 {
                return 0.0;
            }
        }
    }

    vis.clamp(0.0, 1.0)
}

/// Pequeño offset para evitar auto-colisión (acné). Empuja el origen
/// a lo largo de la normal dependiendo del sentido del rayo.
#[inline]
fn offset_origin(p: glm::Vec3, n: glm::Vec3, dir: glm::Vec3) -> glm::Vec3 {
    let sign = if glm::dot(&dir, &n) >= 0.0 { 1.0 } else { -1.0 };
    p + n * (sign * RAY_BIAS)
}

#[inline]
fn mix3(a: Color, b: Color, c: Color, wa: f32, wb: f32, wc: f32) -> Color {
    // Normaliza en caso de redondeos
    let s = (wa + wb + wc).max(1e-6);
    let wa = wa / s; let wb = wb / s; let wc = wc / s;

    let av = a.to_vec3();
    let bv = b.to_vec3();
    let cv = c.to_vec3();
    Color::from_vec3(&(wa * av + wb * bv + wc * cv))
}