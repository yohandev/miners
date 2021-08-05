use crate::world::Block;
// use crate::world::blockdef;

use super::WoodVariant;

// blockdef!
// {
//     id: "wooden_planks",
//     name: |self| { format!("{} Planks", self.variant) },

    #[derive(Block, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockWoodenPlanks
    {
        /// The type wooden planks
        #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
        pub variant: WoodVariant,
    }
//}