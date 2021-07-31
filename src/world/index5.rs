// use std::{any::{Any, TypeId}, borrow::Borrow, ops::Deref};

// use crate::util::Bits;

// trait Block: Any
// {
//     fn unpack(state: Bits<6>) -> Self where Self: Sized;

//     fn unpack_dyn(state: Bits<6>) -> Box<dyn Block> where Self: Sized
//     {
//         Box::new(Self::unpack(state))
//     }
// }

// impl dyn Block
// {
//     pub fn is<T: Block>(&self) -> bool 
//     {
//         // Get `TypeId` of the type this function is instantiated with.
//         let t = TypeId::of::<T>();

//         // Get `TypeId` of the type in the trait object (`self`).
//         let concrete = self.type_id();

//         // Compare both `TypeId`s on equality.
//         t == concrete
//     }

//     #[inline]
//     pub fn downcast_ref<T: Block>(&self) -> Option<&T>
//     {
//         if self.is::<T>()
//         {
//             // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
//             // that check for memory safety because we have implemented Any for all types; no other
//             // impls can exist as they would conflict with our impl.
//             unsafe { Some(&*(self as *const dyn Block as *const T)) }
//         } else {
//             None
//         }
//     }
// }

// enum BlockRef<'a, T: Borrow<dyn Block>>
// {
//     Data(Box<dyn Block>),
//     Addr(&'a dyn Block),
// }

// type BlockRefAny<'a> = BlockRef<'a, Box<dyn Block>>;

// impl<'a> BlockRef<'a>
// {
//     fn downcast<T: Block>(&self) -> Option<&T>
//     {
//         match self
//         {
//             BlockRef::Data(block) => block.downcast_ref(),
//             BlockRef::Addr(block) => block.downcast_ref(),
//         }
//     }
// }

// struct Chunk
// {
//     blocks: [Bits<6>; 32usize.pow(3)],
//     addr: Vec<Box<dyn Block>>
// }

// impl Chunk
// {
//     fn get(&self, pos: (i32, i32, i32)) -> Option<BlockRef<'_>>
//     {
//         todo!()
//     }

//     //fn get_typed<T: Block>(&self, pos: (i32, i32, i32))
// }