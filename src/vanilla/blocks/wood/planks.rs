use crate::world::block::{ Block, self };
// use crate::world::blockdef;

use super::WoodVariant;

// blockdef!
// {
//     id: "wooden_planks",
//     name: |self| { format!("{} Planks", self.variant) },

    #[derive(block::State, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockWoodenPlanks
    {
        /// The type wooden planks
        #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
        pub variant: WoodVariant,
    }

    impl Block for BlockWoodenPlanks
    {
        const ID: &'static str = "wooden_planks";

        fn name(&self) -> std::borrow::Cow<'static, str>
        {
            format!("{} Planks", self.variant).into()
        }
    }
//}