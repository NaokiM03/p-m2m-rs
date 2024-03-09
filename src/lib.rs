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

    /// Inserts a left-right pair into the m2m.
    ///
    /// If the m2m did not previously contain this value, `true` is returned.
    ///
    /// If the m2m already contained this value, `false` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::new();
    ///
    /// assert!(m2m.insert(1, "a"));
    /// assert!(m2m.insert(1, "b"));
    /// assert!(m2m.insert(2, "a"));
    /// assert!(m2m.insert(2, "b"));
    ///
    /// assert!(!m2m.insert(1, "a"));
    /// ```
    pub fn insert(&mut self, left: L, right: R) -> bool
    where
        L: PartialEq,
        R: PartialEq,
    {
        let value = (left, right);

        if self.0.contains(&value) {
            false
        } else {
            self.0.push(value);
            true
        }
    }

    /// Clears the m2m, removing all left-right pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::new();
    ///
    /// assert!(m2m.insert(1, "a"));
    /// assert!(m2m.insert(1, "b"));
    ///
    /// m2m.clear();
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns `true` if the m2m contains no pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::new();
    /// assert!(m2m.is_empty());
    /// m2m.insert(1, "a");
    /// assert!(!m2m.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
