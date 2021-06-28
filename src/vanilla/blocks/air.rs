use crate::world::blockdef;

blockdef!
{
    id: "air",
    name: |self| "Air",

    /// The default "empty" block, with the state properties.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockAir { }
}