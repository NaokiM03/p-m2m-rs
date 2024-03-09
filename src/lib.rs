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
    /// assert!(!m2m.is_empty());
    /// m2m.clear();
    /// assert!(m2m.is_empty());
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

    /// Returns the number of pairs in the m2m.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::new();
    /// assert_eq!(m2m.len(), 0);
    /// m2m.insert(1, "a");
    /// assert_eq!(m2m.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Removes some pairs from the m2m, returning the right values paird with the left if the left was previously in the m2m.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::new();
    /// assert!(m2m.is_empty());
    ///
    /// m2m.insert(1, "a");
    ///
    /// assert_eq!(m2m.remove(&1), Some(vec!["a"]));
    /// assert_eq!(m2m.remove(&1), None);
    ///
    /// assert!(m2m.is_empty());
    /// ```
    pub fn remove(&mut self, left: &L) -> Option<Vec<R>>
    where
        L: PartialEq,
    {
        let mut rights = Vec::new();

        let mut i = 0;
        while i < self.0.len() {
            if &self.0[i].0 == left {
                let lr = self.0.remove(i);
                rights.push(lr.1);
            } else {
                i += 1;
            }
        }

        if rights.is_empty() {
            None
        } else {
            Some(rights)
        }
    }

    /// Returns `true` if the m2m contains the specified left-right pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::new();
    ///
    /// m2m.insert(1, "a");
    ///
    /// assert!(m2m.contains(&1, &"a"));
    /// assert!(!m2m.contains(&1, &"b"));
    /// ```
    pub fn contains(&self, left: &L, right: &R) -> bool
    where
        L: PartialEq,
        R: PartialEq,
    {
        self.0.iter().any(|(l, r)| l == left && r == right)
    }
}
