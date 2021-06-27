use super::*;

#[block("res/blocks/air.toml")]
pub struct BlockAir;

#[block("res/blocks/stairs.toml")]
pub struct BlockStairs(Direction, WoodVariant);