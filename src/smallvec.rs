use core::fmt::{Debug, Formatter, Result};

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
