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

impl<L: Ord, R: Ord, A: Array<Item = (L, R)>> FromIterator<(L, R)> for SmallM2M<A> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = (L, R)>>(iter: I) -> Self {
        let mut v: SmallVec<A> = iter.into_iter().collect();

        v.sort();
        v.dedup();

        SmallM2M(v)
    }
}
