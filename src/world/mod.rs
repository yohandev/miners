pub mod block;
pub mod chunk;
// mod world;

// pub use _block::{ Block, BlockId, BlockState, BlockRepr, BlockRegistry, BlockMeta };
pub use block::{ Block, /*Ref as BlockRef RefMut as BlockRefMut*/ };
// pub use chunk::Chunk;
// pub use world::World;

// pub use crate::blockdef;

// #[cfg(test)]
// mod test
// {
//     use std::sync::Arc;

//     use crate::blocks::{ BlockAir, BlockChest, BlockGrass };
//     use crate::blocks::props::Facing;
//     use crate::math::vec3;

//     use super::*;

//     #[test]
//     fn chunk_storage()
//     {
//         let mut registry = BlockRegistry::default();

//         registry.insert::<BlockAir>();
//         registry.insert::<BlockGrass>();
//         registry.insert::<BlockChest>();

//         let mut chunk = Chunk::new(Arc::new(registry));

//         dbg!(chunk.get::<BlockAir>(vec3(0, 0, 0)));

//         chunk.set(vec3(0, 0, 0), BlockGrass);

//         dbg!(chunk.get::<BlockAir>(vec3(0, 0, 0)));
//         dbg!(chunk.get::<BlockGrass>(vec3(0, 0, 0)));

//         chunk.set(vec3(0, 0, 0), BlockChest { inventory: vec!["Stick x64".into()], facing: Facing::East });

//         dbg!(chunk.get::<BlockGrass>(vec3(0, 0, 0)));
//         dbg!(chunk.get::<BlockChest>(vec3(0, 0, 0)));

//         chunk.set(vec3(0, 0, 0), BlockGrass);

//         dbg!(chunk.get::<BlockChest>(vec3(0, 0, 0)));
//         dbg!(chunk.get::<BlockGrass>(vec3(0, 0, 0)));
//     }
// }