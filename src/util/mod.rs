mod bits;

pub use bits::Bits;

/// A string type that is either statically bundled in the
/// program binary(`&'static str`) or created at runtime(`String`).
///
/// This is useful in "display name"-like functions, where some
/// items have non-changing display names and others are impossible
/// to hard code.
///
/// Usage:
/// ```
/// // Dirt::_
/// fn name(&self) -> StaticString
/// {
///     // Doesn't matter what `&self` contains, dirt is dirt!
///     "Dirt".into()
/// }
/// // Chest::_
/// fn name(&self) -> StaticString
/// {
///     // Who could possibly own this chest?
///     format!("{}'s Chest", self.owner).into()
/// }
/// ```
pub type StaticString = std::borrow::Cow<'static, String>;