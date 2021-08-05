pub mod block;
mod chunk;
mod world;

pub use block::Block;
pub use chunk::Chunk;
pub use world::World;

// pub use crate::blockdef;

#[cfg(test)]
mod test
{
    use std::sync::Arc;

    use crate::math::{ Direction, vec3 };
    use crate::world::{ Chunk, block };
    use crate::vanilla::blocks::*;

    #[test]
    fn chunk_storage()
    {
        let mut registry = block::Registry::default();

        registry.register::<BlockAir>();
        registry.register::<BlockWoodenPlanks>();
        registry.register::<BlockWoodenSlab>();
        registry.register::<BlockChest>();

        let mut chunk = Chunk::new(Arc::new(registry));

        dbg!(chunk.get(vec3(0, 0, 0)).map(|b| b.name()));

        let _a = chunk.get_mut(vec3(0, 0, 0)).unwrap();
        
        chunk.set(vec3(0, 0, 0), BlockWoodenPlanks { variant: WoodVariant::Birch });
        
        dbg!(chunk.get(vec3(0, 0, 0)).map(|b| b.name()));

        chunk.set(vec3(0, 0, 0), BlockChest
            {
                facing: Direction::North,
                contents: vec!["Stick x64", "Diamonds x3"]
            }
        );
        
        dbg!(chunk.get(vec3(0, 0, 0)).map(|b| b.name()));

        chunk.set(vec3(1, 0, 0), BlockChest
            {
                facing: Direction::North,
                contents: vec!["Dirt x12"]
            }
        );
        chunk.set(vec3(0, 1, 0), BlockWoodenPlanks { variant: WoodVariant::Birch });

        dbg!(chunk.get(vec3(0, 0, 0)).map(|b| b.name()));
        dbg!(chunk.get(vec3(1, 0, 0)).map(|b| b.name()));
        dbg!(chunk.get(vec3(0, 1, 0)).map(|b| b.name()));
        dbg!(chunk.get(vec3(0, 10, 0)).map(|b| b.name()));

        dbg!(chunk[(0, 0, 0)].cast::<BlockAir>());
        dbg!(chunk[(1, 0, 0)].cast::<BlockAir>());
        dbg!(chunk[(0, 1, 0)].cast::<BlockAir>());
        dbg!(chunk[(0, 10, 0)].cast::<BlockAir>());

        dbg!(chunk[(0, 0, 0)].cast::<BlockWoodenPlanks>());
        dbg!(chunk[(1, 0, 0)].cast::<BlockWoodenPlanks>());
        dbg!(chunk[(0, 1, 0)].cast::<BlockWoodenPlanks>());
        dbg!(chunk[(0, 10, 0)].cast::<BlockWoodenPlanks>());
    }
}