// use crate::world::blockdef;
use crate::world::Block;

// blockdef!
// {
//     id: "air",
//     name: |self| "Air",

    /// The default "empty" block, with the state properties.
    #[derive(Block, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockAir { }
// }