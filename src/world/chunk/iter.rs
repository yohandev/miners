use crate::world::block;
use crate::world::Chunk;
use crate::math::Vec3;

/// An iterator over a [Chunk]
pub struct Iter<'a>
{
    /// The [Chunk] being iterated
    chunk: &'a Chunk,
    /// Next (flat) block index
    next: usize,
}

impl Chunk
{
    /// Iterate over all of this [Chunk]'s block
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a>
    {
        Iter
        {
            chunk: self,
            next: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Chunk
{
    type Item = (Vec3<usize>, &'a dyn block::Object);
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter
    {
        self.iter()
    }
}

impl<'a> Iterator for Iter<'a>
{
    type Item = (Vec3<usize>, &'a dyn block::Object);

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.next < Chunk::VOLUME
        {
            let pos = Vec3::new(
                self.next & 0x1f,
                (self.next >> 5) & 0x1f,
                self.next >> 10,
            );
            // SAFETY:
            // `self.next` is guarenteed to be in-bounds, checked above
            let block = unsafe { self.chunk.get_unchecked_flat(self.next) }.unwrap();
            self.next += 1;

            Some((pos, block))
        }
        else { None }
    }
}