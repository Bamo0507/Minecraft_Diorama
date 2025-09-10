use std::collections::HashMap;
use nalgebra_glm as glm;
use crate::core::geometry::cube::Cube;
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

    /// Transforma los bloques a Cubes “de mundo” (1 unidad por bloque)
    pub fn bake(&self, reg: &MaterialRegistry) -> Vec<Cube> {
        let mut out = Vec::with_capacity(self.blocks.len());
        for (&(x, y, z), b) in &self.blocks {
            if b.kind == BlockKind::Air { continue; }
            if let Some(mat) = reg.get(b.kind) {
                let min = glm::vec3(x as f32, y as f32, z as f32);
                let max = min + glm::vec3(1.0, 1.0, 1.0);
                out.push(Cube::new(min, max, mat.clone()));
            }
        }
        out
    }
}