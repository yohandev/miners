mod props;
mod blocks;

use std::borrow::Cow;

pub use miners_macros::*;
pub use blocks::*;
pub use props::*;

use crate::util::Bits;

pub trait Block: BlockState + 'static
{
    fn id() -> &'static str;
    fn namespace() -> &'static str;

    fn name(&self) -> Cow<'static, str>;
}

pub trait BlockState
{
    fn serialize(&self) -> Bits<6>;
    fn deserialize(state: Bits<6>) -> Self;
}

#[derive(BlockState, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockAir;

#[derive(BlockState, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockWoodenSlab
{
    /// Direction the slab is oriented, where down means a lower,
    /// horizontal half-slab and north indicates a vertical half-slab
    /// with its largest face touching the north side of the block
    /// boundary.
    #[prop(North | South | East | West | Up | Down)]
    facing: Direction,
    /// The type of wood this slab is made of.
    #[prop(Oak | Spruce | Birch | Jungle | Acacia | DarkOak)]
    variant: WoodVariant,
}

#[derive(BlockState, Debug, Clone, PartialEq, Eq)]
pub struct BlockChest
{
    #[prop(North | South | East | West)]
    facing: Direction,
    #[prop(!)]
    contents: Vec<String>,
}

#[derive(BlockState, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockRedstoneDust
{
    #[prop(North | South | East | West)]
    pub facing: Direction,
    #[prop(0..16)]
    pub power: u8,
}

impl Block for BlockWoodenSlab
{
    fn id() -> &'static str
    {
        "wooden_slab"
    }

    fn namespace() -> &'static str
    {
        "vanilla"
    }

    fn name(&self) -> Cow<'static, str>
    {
        format!("{:?} Slab", self.variant).into()
    }
}

// pub enum BlockState<T>
// {
//     Addr(T),
//     Data(Bits<6>)
// }

// struct BlockPoop;

// impl Block for BlockPoop
// {
//     type State = Bits<6>;

//     fn serialize(&self, _: &mut Bits<6>) { }
//     fn deserialize(_: Bits<6>) -> Self { BlockPoop }
// }

// pub trait BlockState { }

// impl<T: Block<State = T>> BlockState for T { }
// impl BlockState for Bits<6> { }