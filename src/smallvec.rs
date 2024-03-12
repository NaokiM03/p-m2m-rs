use smallvec::{Array, SmallVec};

pub struct SmallM2M<A: Array>(SmallVec<A>);
