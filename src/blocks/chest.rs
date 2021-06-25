use crate::world::Block;

use super::props::Facing;

pub struct BlockChest
{
    // dummy
    pub inventory: Vec<String>,
    pub facing: Facing,
}

impl Block for BlockChest
{
    fn unpack(_: u8) -> Self where Self: Sized
    {
        unreachable!()
    }

    fn pack(&self) -> Option<u8>
    {
        None
    }
}