use crate::math::Direction;
use crate::world::Block;

pub type Inventory = Vec<&'static str>;

#[derive(Block, Debug, Clone, PartialEq, Eq)]
pub struct BlockChest
{
    /// Items in this chest
    #[prop(!)]
    pub contents: Inventory,
    /// Which side the buckle of this chest is facing
    #[prop(North | South | East | West)]
    pub facing: Direction,
}