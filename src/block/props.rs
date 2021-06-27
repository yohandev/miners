/// Enumerates over the six, axis aligned directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction
{
    /// `-Z` Direction
    North,
    /// `+Z` Direction
    South,
    /// `+X` Direction
    East,
    /// `-X` Direction
    West,
    /// `+Y` Direction
    Up,
    /// `-Y` Direction
    Down,
}

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