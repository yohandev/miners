use crate::world::blockdef;

blockdef!
{
    id: "air",
    name: "Air",

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BlockAir;
}