use crate::world::block::{ Block, self };
use crate::math::Direction;

pub type Inventory = Vec<&'static str>;

#[derive(block::State, Debug, Clone, PartialEq, Eq)]
pub struct BlockChest
{
    /// Items in this chest
    #[prop(!)]
    pub contents: Inventory,
    /// Which side the buckle of this chest is facing
    #[prop(North | South | East | West)]
    pub facing: Direction,
    /// This chest's custom name
    #[prop(!)]
    pub name: Option<String>,
}

impl Block for BlockChest
{
    const ID: &'static str = "chest";

    fn name(&self) -> std::borrow::Cow<'static, str>
    {
        match &self.name
        {
            Some(name) => name.clone().into(),
            None => "Chest".into(),
        }
    }
}