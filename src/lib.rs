use core::slice::{Iter, IterMut};

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

impl<L, R> FromIterator<(L, R)> for M2M<L, R>
where
    L: Ord,
    R: Ord,
{
    #[inline]
    fn from_iter<T: IntoIterator<Item = (L, R)>>(iter: T) -> Self {
        let mut v: Vec<(L, R)> = iter.into_iter().collect();
        v.sort();
        v.dedup();
        M2M(v)
    }
}

impl<L, R, const N: usize> From<[(L, R); N]> for M2M<L, R>
where
    L: Ord,
    R: Ord,
{
    /// Converts to this type from the input type.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    /// ```

    fn from(value: [(L, R); N]) -> Self {
        M2M::from_iter(value)
    }
}

impl<L, R> M2M<L, R> {
    /// Creates an empty M2M
    pub fn new() -> M2M<L, R> {
        Default::default()
    }

    /// Inserts a left-right pair into the m2m.
    ///
    /// If the m2m did not previously contain this pair, `true` is returned.
    ///
    /// If the m2m already contained this pair, `false` is returned.
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
    /// let mut m2m = M2M::from([(1, "a"), (1, "b")]);
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
    ///
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
    ///
    /// assert_eq!(m2m.len(), 0);
    /// m2m.insert(1, "a");
    /// assert_eq!(m2m.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Removes some pairs from the m2m,
    /// returning the right values corresponding to the left if the left was previously in the m2m.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a")]);
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
    /// let mut m2m = M2M::from([(1, "a")]);
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

    /// Returns an iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(1, "a")));
    /// assert_eq!(iter.next(), Some(&(1, "b")));
    /// assert_eq!(iter.next(), Some(&(2, "a")));
    /// assert_eq!(iter.next(), Some(&(2, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<(L, R)> {
        self.0.iter()
    }

    /// Returns a mutable iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// m2m.iter_mut().for_each(|(l, r)| *l += 2);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(3, "a")));
    /// assert_eq!(iter.next(), Some(&(3, "b")));
    /// assert_eq!(iter.next(), Some(&(4, "a")));
    /// assert_eq!(iter.next(), Some(&(4, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<(L, R)> {
        self.0.iter_mut()
    }

    /// Extract a slice containing all pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b")]);
    ///
    /// let slice = m2m.as_slice();
    ///
    /// assert_eq!(slice[0], (1, "a"));
    /// assert_eq!(slice[1], (1, "b"));
    /// ```
    pub fn as_slice(&self) -> &[(L, R)] {
        self.0.as_slice()
    }
}

impl<L, R> M2M<L, R> {
    /// Returns a reference to the right values corresponding to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b"), (2, "c"), (2, "d")]);
    ///
    /// let rights = m2m.get_rights(&1);
    /// assert_eq!(rights, Some(vec![&"a", &"b"]));
    /// ```
    pub fn get_rights(&self, left: &L) -> Option<Vec<&R>>
    where
        L: PartialEq,
    {
        let rights: Vec<&R> = self
            .0
            .iter()
            .filter(|(l, _)| l == left)
            .map(|(_, r)| r)
            .collect();

        if rights.is_empty() {
            None
        } else {
            Some(rights)
        }
    }

    /// Returns a reference to the left values corresponding to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (2, "b"), (3, "a"), (4, "b")]);
    ///
    /// let lefts = m2m.get_lefts(&"a");
    /// assert_eq!(lefts, Some(vec![&1, &3]));
    /// ```
    pub fn get_lefts(&self, right: &R) -> Option<Vec<&L>>
    where
        R: PartialEq,
    {
        let lefts: Vec<&L> = self
            .0
            .iter()
            .filter(|(_, r)| r == right)
            .map(|(l, _)| l)
            .collect();

        if lefts.is_empty() {
            None
        } else {
            Some(lefts)
        }
    }

    /// Returns `true` if the m2m contains the specified left value.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b")]);
    ///
    /// assert!(m2m.contains_left(&1));
    /// assert!(!m2m.contains_left(&3));
    /// ```
    pub fn contains_left(&self, left: &L) -> bool
    where
        L: PartialEq,
    {
        self.0.iter().any(|(l, _)| l == left)
    }

    /// Returns `true` if the m2m contains the specified right value.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b")]);
    ///
    /// assert!(m2m.contains_right(&"a"));
    /// assert!(!m2m.contains_right(&"c"));
    /// ```
    pub fn contains_right(&self, right: &R) -> bool
    where
        R: PartialEq,
    {
        self.0.iter().any(|(_, r)| r == right)
    }
}
