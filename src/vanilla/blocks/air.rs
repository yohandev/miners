// use crate::world::blockdef;
use crate::world::block;

// blockdef!
// {
//     id: "air",
//     name: |self| "Air",

    /// The default "empty" block, with the state properties.
    #[derive(block::State, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockAir { }
// }