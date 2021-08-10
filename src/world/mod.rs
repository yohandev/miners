pub mod block;
mod chunk;
mod world;

pub use block::{ Block, blockdef };
pub use chunk::Chunk;
pub use world::World;

#[cfg(test)]
mod tests
{
    use crate::world::{ World, block };
    use crate::vanilla::blocks::*;
    use crate::math::vec3;

    #[test]
    fn test_world()
    {
        let mut registry = block::Registry::default();

        registry.register::<BlockAir>();
        registry.register::<BlockChest>();
        registry.register::<BlockWoodenPlanks>();
        registry.register::<BlockWoodenSlab>();

        let mut world = World::new(registry);

        assert_eq!(world.num_chunks_loading(), 0);
        assert!(matches!(world.get(vec3(0, 0, 0)), None));

        println!("start loading chunks...");
        for x in 0..12
        {
            for z in 0..12
            {
                world.load_chunk(vec3(x * 32, 0, z * 32));
            }
        }

        // wait for chunk(s) to load
        while world.num_chunks_loading() != 0 { }
        
        println!("done loading chunks...");

        println!("world[0, 0, 0] = {:?}", world.get(vec3(0, 0, 0)).map(|b| b.name()));
        println!("world[1, 0, 0] = {:?}", world.get(vec3(1, 0, 0)).map(|b| b.name()));
        assert!(world.get(vec3(0, 0, 0)).unwrap().is::<BlockAir>());
    }
}

/* 
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
                contents: vec!["Stick x64", "Diamonds x3"],
                name: None
            }
        );
        
        dbg!(chunk.get(vec3(0, 0, 0)).map(|b| b.name()));

        chunk.set(vec3(1, 0, 0), BlockChest
            {
                facing: Direction::North,
                contents: vec!["Dirt x12"],
                name: Some("_nahoy's Chest".into()),
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
        dbg!(*chunk[(0, 1, 0)].cast::<BlockWoodenPlanks>().unwrap());
        dbg!(chunk[(0, 10, 0)].cast::<BlockWoodenPlanks>());

        if let Some(mut planks) = chunk[(0, 1, 0)].cast_mut::<BlockWoodenPlanks>()
        {
            println!("BEFORE: {:?}", planks);

            planks.variant = WoodVariant::Oak;

            println!("AFTER: {:?}", planks);
        };
        println!("PRESERVED?: {:?}", chunk[(0, 1, 0)].cast_mut::<BlockWoodenPlanks>().unwrap());
    
        for (_, block) in &chunk
        {
            println!("{}", block.name());
        }
    }
} */