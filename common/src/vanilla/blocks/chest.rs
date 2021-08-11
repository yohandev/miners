use std::borrow::Cow;

use crate::world::blockdef;
use crate::math::Direction;

pub type Inventory = Vec<&'static str>;

blockdef!
{
    id: "chest",
    name: match &self.name
    {
        Some(x) => Cow::Owned(x.clone()),
        None => Cow::Borrowed("chest"),
    },

    #[derive(Debug, Clone, PartialEq, Eq)]
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
}