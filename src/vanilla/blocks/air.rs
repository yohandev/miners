// use crate::world::blockdef;
use crate::world::block::{ Block, self };

// blockdef!
// {
//     id: "air",
//     name: |self| "Air",

    /// The default "empty" block, with the state properties.
    #[derive(block::State, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockAir { }

    impl Block for BlockAir
    {
        const ID: &'static str = "air";

        fn name(&self) -> std::borrow::Cow<'static, str> { "Air".into() }
    }
// }