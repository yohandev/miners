use crate::util::Bits;

use super::Block;

pub enum Ref<'a>
{
    Data(Bits<6>),
    Addr(&'a dyn Block)
}

impl<'a> Ref<'a>
{
    pub fn visit<T>(&self, visitor: impl FnOnce(&dyn Block) -> T) -> T
    {
        match self
        {
            // Data references need to be deserialized on the
            // stack and be outlived by the visitor closure.
            Ref::Data(_) => todo!(),
            // Addr references are already `&dyn Block`
            Ref::Addr(block) => visitor(*block),
        }
    }
}