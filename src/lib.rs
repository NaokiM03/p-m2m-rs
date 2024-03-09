/// A many-to-many implemented as a `Vec<(L, R)>`.
///
/// M2M is just a wrapper around a Vec.
#[derive(Debug)]
pub struct M2M<L, R>(Vec<(L, R)>);

impl<L, R> Default for M2M<L, R> {
    #[inline]
    fn default() -> Self {
        M2M(Vec::new())
    }
}

impl<L, R> M2M<L, R> {
    /// Creates an empty M2M
    pub fn new() -> M2M<L, R> {
        Default::default()
    }
}
