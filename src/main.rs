mod app;
mod core;

use nalgebra_glm as glm;
use app::window::Window;
use core::color::Color;
use core::framebuffer::Framebuffer;
use core::geometry::aabb::Aabb;
use core::renderer::Renderer;
use core::scene::Scene;
use core::camera::OrbitCamera;
use core::light::Light;

// Mundo / materiales / texturas
use core::texture::Texture;
use core::material::Material;
use core::material_registry::MaterialRegistry;
use core::block::BlockKind;
use core::world::World;
use core::skybox::Skybox;

use raylib::prelude::MouseButton;

fn main() {
    let width = 1300;
    let height = 900;

    // Ventana
    let mut window = Window::new("Minecraft Diorama RT", width as usize, height as usize);

    // Framebuffer
    let mut fb = Framebuffer::new(width as u32, height as u32);

    // Cámara orbital (centro entre capas para ver ambos niveles)
    let mut camera = OrbitCamera::new(glm::vec3(0.0, 2.0, 0.0), 10.0, 1.0, 0.35);

    // ---------------------------
    //  Texturas (assets/*.png)
    // ---------------------------
    let grass_top   = Texture::load("assets/grass_top.png");
    let grass_side  = Texture::load("assets/grass_side.png").rotated_180();
    let dirt_tex    = Texture::load("assets/dirt.png");
    let stone_tex   = Texture::load("assets/stone.png");
    let iron_tex    = Texture::load("assets/iron.png");
    let diamond_tex = Texture::load("assets/diamond.png");
    let lava_tex    = Texture::load("assets/lava.png");
    let water_tex   = Texture::load("assets/water.png");
    let wood_tex   = Texture::load("assets/wood.png");
    let leaves_tex = Texture::load("assets/leaves.png");

    let sky_px = Texture::load("assets/sky_px.png");
    let sky_nx = Texture::load("assets/sky_nx.png");
    let sky_py = Texture::load("assets/sky_py.png");
    let sky_ny = Texture::load("assets/sky_ny.png");
    let sky_pz = Texture::load("assets/sky_pz.png");
    let sky_nz = Texture::load("assets/sky_nz.png");

    let skybox = Skybox::new(sky_px, sky_nx, sky_py, sky_ny, sky_pz, sky_nz);

    // ------------------------------------------------------
    //  Materiales con texturas y parámetros independientes
    // ------------------------------------------------------
    // Grass: 6 caras (side/side/bottom/top/side/side)
    let grass_mat = Material::with_cube_textures(
        grass_side.clone(), grass_side.clone(), // nx, px
        dirt_tex.clone(),   grass_top.clone(),  // ny (bottom), py (top)
        grass_side.clone(), grass_side.clone(), // nz, pz
        0.10, 12.0, 0.02, 0.0, 1.0              // specular, shininess, reflectivity, transparency, ior
    );

    // Dirt
    let dirt_mat = Material::with_texture(
        dirt_tex.clone(),
        0.02, 6.0, 0.00, 0.0, 1.0
    );

    // Stone
    let stone_mat = Material::with_texture(
        stone_tex.clone(),
        0.03, 8.0, 0.00, 0.0, 1.0
    );

    // Iron ore
    let iron_mat = Material::with_texture(
        iron_tex.clone(),
        0.15, 24.0, 0.05, 0.0, 1.0
    );

    // Diamond ore
    let diamond_mat = Material::with_texture(
        diamond_tex.clone(),
        0.20, 32.0, 0.08, 0.0, 1.0
    );

    // Lava (opaca por ahora; más adelante podemos darle emisión/transparencia)
    let lava_mat = Material::with_texture(
        lava_tex.clone(),
        0.25, 32.0, 0.02, 0.0, 1.0
    );

    // Agua
    let water_mat = Material::with_texture(
        water_tex.clone(),
        0.18,   // specular
        64.0,   // shininess
        0.9,
        0.66,   // transparency (↑ para que se vea la refracción)
        1.33    // ior del agua
    );

    // Madera (opaca, poco especular)
    let wood_mat = Material::with_texture(
        wood_tex.clone(),
        0.05, 16.0, 0.02, 0.0, 1.0
    );

    // Hojas (ligeramente transparentes; IOR ~1.33)
    let leaves_mat = Material::with_texture(
        leaves_tex.clone(),
        0.10, 12.0, 0.00, 0.30, 1.33
    );
        

    // Registry base (Grass/Dirt/Stone). Iron/Diamond/Lava los pondremos por posición post-bake.
    let mut registry = MaterialRegistry::new();
    registry.set(BlockKind::Grass, grass_mat.clone());
    registry.set(BlockKind::Dirt,  dirt_mat.clone());
    registry.set(BlockKind::Stone, stone_mat.clone());
    registry.set(BlockKind::Water, water_mat.clone());
    registry.set(BlockKind::Wood,   wood_mat.clone());
    registry.set(BlockKind::Leaves, leaves_mat.clone());

    // ---------------------------
    //  Construcción del diorama
    // ---------------------------
    let mut world = World::new();

    // Piso superior 5x5, con grass y water
    for z in -2..=2 {
        for x in -2..=2 {
            let is_border = x == -2 || x == 2 || z == -2 || z == 2;
            let is_center = x == 0 && z == 0;
            if is_border || is_center {
                world.set(x, 0, z, BlockKind::Grass);
            } else {
                world.set(x, 0, z, BlockKind::Water);
            }
        }
    }

    // Tronco del arbol
    for y in 1..=3 {
        world.set(0, y, 0, BlockKind::Wood);
    }

    // Copa de hojas
    for dz in -1..=1 {
        for dx in -1..=1 {
            world.set(dx, 4, dz, BlockKind::Leaves);
        }
    }

    world.set(-1, 3,  0, BlockKind::Leaves);
    world.set( 1, 3,  0, BlockKind::Leaves);
    world.set( 0, 3, -1, BlockKind::Leaves);
    world.set( 0, 3,  1, BlockKind::Leaves);

    world.set(0, 5, 0, BlockKind::Leaves);

    // Paredes traseras para rellenar 
    {
        let y_min = -4;
        let y_max = -1;

        let z_back = -2;
        for x in -2..=2 {
            for y in y_min..=y_max {
                world.set(x, y, z_back, BlockKind::Stone);
            }
        }

        let x_back = -2;
        for z in -2..=2 {
            for y in y_min..=y_max {
                world.set(x_back, y, z, BlockKind::Stone);
            }
        }
    }

    // Construccion de columnas, mas para las estacas
    let mut place_column = |x: i32, z: i32, dirt_n: i32, tail_kind: BlockKind, tail_n: i32| {
        let mut y = -1;
        for _ in 0..dirt_n { world.set(x, y, z, BlockKind::Dirt); y -= 1; }
        for _ in 0..tail_n { world.set(x, y, z, tail_kind); y -= 1; }
        // Si aún no llegamos a -5, completa con tail_kind (por seguridad)
        while y >= -5 { world.set(x, y, z, tail_kind); y -= 1; }
    };

    // Columnas
    // 2 dirt + 3 stone
    place_column( 2, -2, 2, BlockKind::Stone, 3);
    // 2 dirt + 3 iron
    place_column(-2,  2, 2, BlockKind::Stone, 3);

    // Piso inferior
    // Donde S=Stone, L=Lava, D=Diamond.
    #[derive(Clone, Copy)]
    enum Cell { S, L, D }

    let pattern: [[Cell; 5]; 5] = [
        [Cell::S, Cell::S, Cell::S, Cell::S, Cell::S],
        [Cell::S, Cell::L, Cell::L, Cell::L, Cell::S],
        [Cell::S, Cell::L, Cell::L, Cell::L, Cell::S],
        [Cell::S, Cell::L, Cell::L, Cell::L, Cell::S],
        [Cell::S, Cell::S, Cell::S, Cell::S, Cell::S],
    ];

    // Esquinas del 5x5 (
    let is_corner = |x: i32, z: i32| (x == -2 || x == 2) && (z == -2 || z == 2);

    // Rango de mundo
    for z in -2..=2 {
        for x in -2..=2 {
            if is_corner(x, z) { continue; } 

            world.set(x, -5, z, BlockKind::Stone);
        }
    }

    // Horneo a AABBs
    let mut aabbs = world.bake(&registry);

    // Sustituciones por posición
    for a in &mut aabbs {
        let x = a.min.x.round() as i32;
        let y = a.min.y.round() as i32;
        let z = a.min.z.round() as i32;

        // Columna de la esquina C 
        if x == -2 && z == 2 && (-5..=-3).contains(&y) {
            a.material = iron_mat.clone();
            continue;
        }

        if x == -2 && (-4..=-1).contains(&y) {
            if z == 2 && (y == -1 || y == -2) {
                a.material = dirt_mat.clone();
                continue;
            }
            if (-1..=1).contains(&z) {
                let col = (z + 1) as i32;
                let row = (-y - 1) as i32;
                let mat = match (row, col) {
                    (0, _         ) => &stone_mat,
                    (1, 0) | (1, 2) => &diamond_mat,
                    (1, 1)         => &stone_mat,
                    (2, 0) | (2, 2) => &stone_mat,
                    (2, 1)         => &diamond_mat,
                    (3, 0) | (3, 2) => &diamond_mat,
                    (3, 1)         => &stone_mat,
                    _               => &stone_mat,
                };
                a.material = mat.clone();
                continue;
            }
        }

        // Cara trasera
        if z == -2 && (-4..=-1).contains(&y) && (-1..=1).contains(&x) {
            let col = (x + 1) as i32;
            let row = (-y - 1) as i32;
            let mat = match (row, col) {
                (0, _         ) => &stone_mat,
                (1, 0) | (1, 2) => &diamond_mat,
                (1, 1)         => &stone_mat,
                (2, 0) | (2, 2) => &stone_mat,
                (2, 1)         => &diamond_mat,
                (3, 0) | (3, 2) => &diamond_mat,
                (3, 1)         => &stone_mat,
                _               => &stone_mat,
            };
            a.material = mat.clone();
            continue;
        }

        // Piso inferior según la matriz
        if y == -5 && !((x == -2 || x == 2) && (z == -2 || z == 2)) {
            let ix = (x + 2) as usize; // -2..2 → 0..4
            let iz = (z + 2) as usize;

            match pattern[iz][ix] {
                Cell::S => { /* ya lo tiene registry */ }
                Cell::L => { a.material = lava_mat.clone(); }
                Cell::D => { a.material = diamond_mat.clone(); }
            }
        }
    }

    // Relleno de esquinas faltantes en el piso inferio
    let corners = [(-2, -2), (2, -2), (-2, 2), (2, 2)];

    for (x, z) in corners {
        // ¿Existe ya un bloque en esa esquina a y=-5?
        let min = glm::vec3(x as f32, -5.0, z as f32);
        let exists = aabbs.iter().any(|a| {
            (a.min.x - min.x).abs() < 1e-6 &&
            (a.min.y - min.y).abs() < 1e-6 &&
            (a.min.z - min.z).abs() < 1e-6
        });
        if exists { continue; }

        // Decide el material a partir de la matriz 5x5 (Cell::S/L/D)
        let ix = (x + 2) as usize; // -2..2 → 0..4
        let iz = (z + 2) as usize;
        let mat = match pattern[iz][ix] {
            Cell::S => stone_mat.clone(),
            Cell::L => lava_mat.clone(),
            Cell::D => diamond_mat.clone(),
        };

        aabbs.push(Aabb::new(min, min + glm::vec3(1.0, 1.0, 1.0), mat));
    }

    // Luz
    let light0 = Light::point(glm::vec3( 4.0, 6.0,  4.0), Color::new(255, 255, 255), 1.5);

    // Escena final
    let scene = Scene {
        spheres: vec![],
        aabbs,
        lights: vec![light0],
        skybox: Some(skybox), // ← aquí
    };

    let renderer = Renderer::new();

    // Sensibilidades de cámara
    let rot_sens = 0.005;
    let zoom_sens = 0.1;

    // switches para invertir
    const INVERT_YAW: f32 = -1.0; 
    const INVERT_PITCH: f32 = -1.0; 
    const INVERT_SCROLL: f32 = -1.0; 

    while window.is_open() {
        // Input cámara
        let (dx, dy) = window.mouse_delta();
        if window.is_mouse_down(MouseButton::MOUSE_BUTTON_RIGHT) || window.is_mouse_down(MouseButton::MOUSE_BUTTON_LEFT) {
            // antes: camera.rotate(dx * rot_sens, -dy * rot_sens);
            camera.rotate(INVERT_YAW * dx * rot_sens, INVERT_PITCH * dy * rot_sens);
        }

        let wheel = window.mouse_wheel();
        if wheel.abs() > 0.0 {
            // antes: camera.zoom(1.0 - wheel * zoom_sens);
            camera.zoom(1.0 - (INVERT_SCROLL * wheel) * zoom_sens);
        }

        // Cielo
        fb.clear(Color::new(135, 206, 235));
        renderer.render_frame(&scene, &mut fb, &camera);
        window.present(fb.pixels());
    }
}