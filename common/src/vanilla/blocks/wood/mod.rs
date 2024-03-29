mod planks;
mod slabs;

pub use planks::*;
pub use slabs::*;

/// The variants of wood in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WoodVariant
{
    Oak,
    Spruce,
    Birch,
    Jungle,
    Acacia,
    DarkOak,
}

impl std::fmt::Display for WoodVariant
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", match self
        {
            WoodVariant::Oak => "Oak",
            WoodVariant::Spruce => "Spruce",
            WoodVariant::Birch => "Birch",
            WoodVariant::Jungle => "Jungle",
            WoodVariant::Acacia => "Acacia",
            WoodVariant::DarkOak => "Dark Oak",
        })
    }
}