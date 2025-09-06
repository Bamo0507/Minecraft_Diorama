use std::collections::HashMap;
use crate::core::material::Material;
use crate::core::block::BlockKind;

pub struct MaterialRegistry {
    map: HashMap<BlockKind, Material>,
}

impl MaterialRegistry {
    pub fn new() -> Self { Self { map: HashMap::new() } }
    pub fn set(&mut self, kind: BlockKind, mat: Material) { self.map.insert(kind, mat); }
    pub fn get(&self, kind: BlockKind) -> Option<&Material> { self.map.get(&kind) }
}