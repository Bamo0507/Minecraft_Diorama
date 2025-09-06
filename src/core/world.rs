use std::collections::HashMap;
use nalgebra_glm as glm;
use crate::core::geometry::aabb::Aabb;
use crate::core::material_registry::MaterialRegistry;
use crate::core::block::{Block, BlockKind};

pub struct World {
    blocks: HashMap<(i32, i32, i32), Block>,
}

impl World {
    pub fn new() -> Self { Self { blocks: HashMap::new() } }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, kind: BlockKind) {
        self.blocks.insert((x, y, z), Block::new(kind));
    }

    #[inline]
    pub fn remove(&mut self, x: i32, y: i32, z: i32) {
        self.blocks.remove(&(x, y, z));
    }

    pub fn clear(&mut self) { self.blocks.clear(); }

    /// Llena un volumen rectangular (incluye extremos)
    pub fn fill_box(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32, kind: BlockKind) {
        let (xa, xb) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
        let (ya, yb) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
        let (za, zb) = if z0 <= z1 { (z0, z1) } else { (z1, z0) };
        for x in xa..=xb { for y in ya..=yb { for z in za..=zb {
            self.set(x, y, z, kind);
        }}}
    }

    /// Transforma los bloques a AABBs “de mundo” (1 unidad por bloque)
    pub fn bake(&self, reg: &MaterialRegistry) -> Vec<Aabb> {
        let mut out = Vec::with_capacity(self.blocks.len());
        for (&(x, y, z), b) in &self.blocks {
            if b.kind == BlockKind::Air { continue; }
            if let Some(mat) = reg.get(b.kind) {
                let min = glm::vec3(x as f32, y as f32, z as f32);
                let max = min + glm::vec3(1.0, 1.0, 1.0);
                out.push(Aabb::new(min, max, mat.clone()));
            }
        }
        out
    }
}