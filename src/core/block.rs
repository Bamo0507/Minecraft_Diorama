#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BlockKind {
    Grass,
    Stone,
    Dirt,
    Lava,
    Diamond,
    Air,
    Water,
    Wood,
    Leaves,
}

#[derive(Clone, Copy, Debug)]
pub struct Block { pub kind: BlockKind }
impl Block { pub fn new(kind: BlockKind) -> Self { Self { kind } } }