/// A bit array wrapping over all or parts of a byte(`u8`),
/// providing compile-time assured abstractions over bitwise,
/// array-like indexing operations.
///
/// ```
/// // Only the lower 6 bits can be touched
/// let bits = Bits::<6>(0b0011_1111);
/// 
/// bits.get::<0, 6>(); // ok
/// bits.get::<2, 3>(); // ok
/// bits.get::<4, 1>(); // error! won't compile
/// bits.get::<4, 7>(); // error! won't compile
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bits<const N: usize>(u8);

impl<const N: usize> Bits<N> where Self: Valid
{
    /// Create a new bit array of length `N` wrapping over the given value.
    /// 
    /// Bits "out of bound" are clipped and set to 0
    #[inline]
    pub fn new(val: u8) -> Self
    {
        Self(val & (0xff >> (8 - N)))
    }

    /// Returns a range of the inner byte. Fails to compile if
    /// `START` >= `END`, or if `END` > `N`(length, in bits, of
    /// this bit array).
    #[inline]
    pub fn get<const START: usize, const END: usize>(&self) -> u8
    where
        Literal<START>: LessThan<Literal<END>>,
        Literal<START>: LessThan<Literal<N>>,
        Literal<END>: LessThanOrEqual<Literal<N>>,
    {
        // Let's look at a `Bits<6>` example, storing all set bits:
        // Bits<6>(0b0011_1111)
        //
        // accessing 0..4 would looks like this:
        // 0011_1111
        //   |___|
        //
        // accessing 4..6 woud look like this:
        // 0011_1111
        //        L⅃
        //
        // So, effectively, accessing a range looks like this:
        // - Shift Right(N - END)
        // - Bitwise AND(MaskOf1s(END - START))
        (self.0 >> (N - END)) & (0xff >> (8 - (END - START)))
    }

    /// Set the range in the inner byte to the given value. The upper bits of
    /// the given value are clipped(set to 0) to `END` - `START`. Fails to compile
    /// if `START` >= `END`, or if `END` > `N`(length, in bits, of this bit array).
    pub fn set<const START: usize, const END: usize>(&mut self, val: u8)
    where
        Literal<START>: LessThan<Literal<END>>,
        Literal<START>: LessThan<Literal<N>>,
        Literal<END>: LessThanOrEqual<Literal<N>>,
    {
        // Let's look at a `Bits<6>` example, storing all set bits:
        // Bits<6>(0b0011_1111)
        //
        // Putting `0b0000_0010` in 0..2 would look like this:
        // 0011_1111
        //   L⅃ <- put 10
        // 0010_1111
        //
        // Putting `0b1111_1010` in 2..6 would look like this:
        // 0011_1111
        //      |__| <- put 1010
        // 0011_1010
        //
        // So, effectively, settings a range looks like this:
        // - Bitwise AND(MaskOf0s(Desired Range))
        // - On `val`, Bitwise AND(MaskOf1s(END - START))
        // - On `val` Shift Left(N - END)
        // - Bitwise OR(`val`)
        let mask = 0xff >> (8 - (END - START));
        let shift = N - END;

        self.0 &= !(mask << shift);
        self.0 |= (val & mask) << shift;
    }

    /// Get the byte this bit array wraps over
    #[inline]
    pub fn inner(self) -> u8
    {
        self.0
    }
}

/// Dummy trait that converts number literals(`0`, `1`, implemented for up to `7`)
/// into concrete types
pub struct Literal<const N: usize>;

/// Dummy trait asserting that the implementing type is less than type `T`
pub trait LessThan<T> { }
/// Dummy trait asserting that the implementing type is less than or equal to type `T`
pub trait LessThanOrEqual<T> { }
/// Dummy trait restricting generic value `N` in `Bits` from `0` to `8`
pub trait Valid { }

// Wall of doom
impl LessThan<Literal<7>> for Literal<0> { }
impl LessThan<Literal<6>> for Literal<0> { }
impl LessThan<Literal<5>> for Literal<0> { }
impl LessThan<Literal<4>> for Literal<0> { }
impl LessThan<Literal<3>> for Literal<0> { }
impl LessThan<Literal<2>> for Literal<0> { }
impl LessThan<Literal<1>> for Literal<0> { }

impl LessThan<Literal<7>> for Literal<1> { }
impl LessThan<Literal<6>> for Literal<1> { }
impl LessThan<Literal<5>> for Literal<1> { }
impl LessThan<Literal<4>> for Literal<1> { }
impl LessThan<Literal<3>> for Literal<1> { }
impl LessThan<Literal<2>> for Literal<1> { }

impl LessThan<Literal<7>> for Literal<2> { }
impl LessThan<Literal<6>> for Literal<2> { }
impl LessThan<Literal<5>> for Literal<2> { }
impl LessThan<Literal<4>> for Literal<2> { }
impl LessThan<Literal<3>> for Literal<2> { }

impl LessThan<Literal<7>> for Literal<3> { }
impl LessThan<Literal<6>> for Literal<3> { }
impl LessThan<Literal<5>> for Literal<3> { }
impl LessThan<Literal<4>> for Literal<3> { }

impl LessThan<Literal<7>> for Literal<4> { }
impl LessThan<Literal<6>> for Literal<4> { }
impl LessThan<Literal<5>> for Literal<4> { }

impl LessThan<Literal<7>> for Literal<5> { }
impl LessThan<Literal<6>> for Literal<5> { }

impl LessThan<Literal<7>> for Literal<6> { }

// blanket implementation saves a bit of headache
impl<const N0: usize, const N1: usize> LessThanOrEqual<Literal<N1>> for Literal<N0> where Literal<N0>: LessThan<Literal<N1>> { }

impl LessThanOrEqual<Literal<7>> for Literal<7> { }
impl LessThanOrEqual<Literal<6>> for Literal<6> { }
impl LessThanOrEqual<Literal<5>> for Literal<5> { }
impl LessThanOrEqual<Literal<4>> for Literal<4> { }
impl LessThanOrEqual<Literal<3>> for Literal<3> { }
impl LessThanOrEqual<Literal<2>> for Literal<2> { }
impl LessThanOrEqual<Literal<1>> for Literal<1> { }
impl LessThanOrEqual<Literal<0>> for Literal<0> { }

impl Valid for Bits<1> { }
impl Valid for Bits<2> { }
impl Valid for Bits<3> { }
impl Valid for Bits<4> { }
impl Valid for Bits<5> { }
impl Valid for Bits<6> { }
impl Valid for Bits<7> { }
impl Valid for Bits<8> { }

#[cfg(test)]
mod test
{
    use super::Bits;

    #[test]
    fn get_range()
    {
        let bits = Bits::<6>::new(0b0011_1111);

        assert_eq!(bits.get::<0, 1>(), 0b0000_0001);
        assert_eq!(bits.get::<0, 4>(), 0b0000_1111);
        assert_eq!(bits.get::<0, 6>(), 0b0011_1111);
        assert_eq!(bits.get::<4, 6>(), 0b0000_0011);

        let bits = Bits::<6>::new(0b0010_1010);

        assert_eq!(bits.get::<0, 1>(), 0b0000_0001);
        assert_eq!(bits.get::<0, 4>(), 0b0000_1010);
        assert_eq!(bits.get::<0, 6>(), 0b0010_1010);
        assert_eq!(bits.get::<4, 6>(), 0b0000_0010);
    }

    #[test]
    fn clip_new()
    {
        let bits = Bits::<6>::new(0b1111_1111);

        assert_eq!(bits.inner(), 0b0011_1111);

        let bits = Bits::<6>::new(0b1100_1100);

        assert_eq!(bits.inner(), 0b0000_1100);
    }

    #[test]
    fn set_range()
    {
        let mut bits = Bits::<6>::new(0b0011_1111);

        bits.set::<0, 2>(0b0000_0010);
        assert_eq!(bits.inner(), 0b0010_1111);

        bits.set::<2, 6>(0b0010_1010);
        assert_eq!(bits.inner(), 0b0010_1010);

        let mut bits = Bits::<6>::new(0);

        bits.set::<0, 6>(0xff);
        assert_eq!(bits.inner(), 0b0011_1111);
    }
}