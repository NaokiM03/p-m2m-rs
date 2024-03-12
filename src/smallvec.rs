use core::fmt::{Debug, Formatter, Result};
use core::iter::FromIterator;

use smallvec::{Array, SmallVec};

pub struct SmallM2M<A: Array>(SmallVec<A>);

impl<A: Array> Debug for SmallM2M<A>
where
    A::Item: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<L, R, A: Array<Item = (L, R)>> FromIterator<(L, R)> for SmallM2M<A>
where
    (L, R): Ord,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = (L, R)>>(iter: I) -> Self {
        let mut v: SmallVec<A> = iter.into_iter().collect();

        v.sort();
        v.dedup();

        SmallM2M(v)
    }
}

impl<L, R, const N: usize, A: Array<Item = (L, R)>> From<[(L, R); N]> for SmallM2M<A>
where
    (L, R): Ord,
{
    /// Converts to this type from the input type.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let m2m: SmallM2M<[(u8, &str); 4]> = SmallM2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    /// ```
    fn from(value: [(L, R); N]) -> Self {
        SmallM2M::from_iter(value)
    }
}
