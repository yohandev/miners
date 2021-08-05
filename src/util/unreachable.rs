#[derive(Copy)]
pub enum Unreachable { }

/// Unsafe function branch to be put wherever code is sure to be unreachable,
/// which the compiler will optimize away. Calling this function("reaching" it)
/// is UB.
#[allow(dead_code)]
pub unsafe fn unreachable() -> !
{
    Unreachable::unreachable(*std::mem::transmute::<usize, &Unreachable>(1))
}

impl Unreachable
{
    /// A function that is statically unreachable, as [Unreachable] itself
    /// cannot be instantiated(that is, within the bounds of safe Rust)
    pub fn unreachable(x: Unreachable) -> !
    {
        match x { }
    }
}

impl Clone for Unreachable
{
    fn clone(&self) -> Self
    {
        Self::unreachable(*self)
    }
}