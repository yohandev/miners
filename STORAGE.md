```rust
// implements Deref<T>, mutable equivalent re-packs its Data variant
// into the chunk on drop
enum BlockRef<'a, T: Block>
{
    Data(T),
    Addr(&'a T)
}

trait Block
{
    fn unpack() -> Option<Self>
    {
        // return None for entity blocks
    }
}

// chunk internally stores a Vec<Box<dyn Block>> for entity
// blocks, and the packed block is an address inside that
// vector.
let block_ref: Option<BlockRef<Chest>> = chunk.get::<Chest>((10, 30, 5));
// packed block can be cast directly to the stairs. internally, chunk
// has a cached 
let block_ref: Option<BlockRef<Stairs>> = chunk.get::<Stairs>((3, 10, 3));
```