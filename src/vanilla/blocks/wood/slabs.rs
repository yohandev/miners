// use crate::world::blockdef;
use crate::world::block::{ Block, self };
use crate::math::Direction;

use super::WoodVariant;

// blockdef!
// {
//     id: "wooden_slab",
//     name: |self| format!("{} Slab", self.variant),

    #[derive(block::State, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockWoodenSlab
    {
        /// Direction the slab is oriented, where down means a lower,
        /// horizontal half-slab and north indicates a vertical half-slab
        /// with its largest face touching the north side of the block
        /// boundary.
        #[prop(North | South | East | West | Up | Down)]
        facing: Direction,
        /// The type wooden slab
        #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
        variant: WoodVariant,
    }

    impl Block for BlockWoodenSlab
    {
        const ID: &'static str = "wooden_slab";

        fn name(&self) -> std::borrow::Cow<'static, str>
        {
            format!("{} Slab", self.variant).into()
        }
    }
// }