use crate::util::Bits;

trait Block: BlockSized
    + Prop<BlockName>
    + Prop<BlockGeometry>
{
}

trait BlockSized: Sized
{
    type Repr: BlockRepr<Self>;
}

trait Prop<T>: BlockSized
{
    fn get(state: &<Self::Repr as BlockRepr<Self>>::State) -> T;
}

trait PropMut<T>: BlockSized
{
    fn get_mut(state: &mut <Self::Repr as BlockRepr<Self>>::State) -> T;
}

trait PropSet<T>: BlockSized
{
    fn set(state: &mut <Self::Repr as BlockRepr<Self>>::State, val: T) -> T;
}

struct BlockReprData;
struct BlockReprAddr;

trait BlockRepr<T>
{
    type State;
}

impl<T: Block> BlockRepr<T> for BlockReprData
{
    type State = Bits<6>;
}

impl<T: Block> BlockRepr<T> for BlockReprAddr
{
    type State = T;
}

enum Ref<'a, T: Block>
{
    Data(<T::Repr as BlockRepr<T>>::State),
    Addr(&'a <T::Repr as BlockRepr<T>>::State),
}

impl<'a, T: Block> Ref<'a, T>
{
    fn get<K>(&self) -> K where T: Prop<K>
    {
        match self
        {
            Ref::Data(state) => T::get(state),
            Ref::Addr(block) => T::get(*block),
        }
    }
}

// Required `Prop`s for a `Block`
struct BlockName(std::borrow::Cow<'static, str>);
enum BlockGeometry
{
    Full,
    Half,
    Stairs,
}

struct BlockWoodenPlanks;

impl Block for BlockWoodenPlanks
{

}

impl BlockSized for BlockWoodenPlanks
{
    type Repr = BlockReprData;
}

impl Prop<BlockName> for BlockWoodenPlanks
{
    fn get(state: &<Self::Repr as BlockRepr<Self>>::State) -> BlockName
    {
        match state.get::<0, 2>()
        {
            1 => BlockName("Oak Planks".into()),
            2 => BlockName("Spruce Planks".into()),
            _ => BlockName("Birch Planks".into()),
        }
    }
}

impl Prop<BlockGeometry> for BlockWoodenPlanks
{
    fn get(_: &<Self::Repr as BlockRepr<Self>>::State) -> BlockGeometry
    {
        BlockGeometry::Full
    }
}

struct BlockChest
{
    name: Option<String>,
    contents: Vec<String>
}

impl Block for BlockChest
{

}

impl BlockSized for BlockChest
{
    type Repr = BlockReprAddr;
}

impl Prop<BlockName> for BlockChest
{
    fn get(state: &<Self::Repr as BlockRepr<Self>>::State) -> BlockName
    {
        match &state.name
        {
            Some(name) => BlockName(name.clone().into()),
            None => BlockName("Chest".into()),
        }
    }
}

impl Prop<BlockGeometry> for BlockChest
{
    fn get(_: &<Self::Repr as BlockRepr<Self>>::State) -> BlockGeometry
    {
        BlockGeometry::Full
    }
}

impl Prop<Vec<String>> for BlockChest
{
    fn get(state: &<Self::Repr as BlockRepr<Self>>::State) -> Vec<String>
    {
        todo!()
    }
}