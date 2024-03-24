use core::fmt::{self, Debug};
use core::iter::{FromIterator, IntoIterator};
use core::slice;

use smallvec::{Array, SmallVec};

/// A many-to-many implemented as a `SmallVec<A>`.
///
/// SmallM2M is just a wrapper around a SmallVec.
pub struct SmallM2M<A: Array>(SmallVec<A>);

impl<A: Array> Debug for SmallM2M<A>
where
    A::Item: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<A: Array> Default for SmallM2M<A> {
    /// Creates an empty `SmallM2M<A>`.
    #[inline]
    fn default() -> Self {
        SmallM2M(SmallVec::new())
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

impl<'a, L, R, A: Array<Item = (L, R)>> IntoIterator for &'a SmallM2M<A>
where
    (L, R): 'a,
{
    type Item = &'a (L, R);
    type IntoIter = slice::Iter<'a, (L, R)>;

    /// Creates an iterator from a value.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let m2m: SmallM2M<[(u8, &str); 4]> = SmallM2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// let mut iter = m2m.into_iter();
    ///
    /// assert_eq!(iter.next(), Some((1, "a")));
    /// assert_eq!(iter.next(), Some((1, "b")));
    /// assert_eq!(iter.next(), Some((2, "a")));
    /// assert_eq!(iter.next(), Some((2, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, L, R, A: Array<Item = (L, R)>> IntoIterator for &'a mut SmallM2M<A>
where
    (L, R): 'a,
{
    type Item = &'a mut (L, R);
    type IntoIter = slice::IterMut<'a, (L, R)>;

    /// Creates an iterator from a value.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let m2m: &mut SmallM2M<[(u8, &str); 4]> =
    ///     &mut SmallM2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// m2m.into_iter().for_each(|(l, _)| *l *= 3);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(3, "a")));
    /// assert_eq!(iter.next(), Some(&(3, "b")));
    /// assert_eq!(iter.next(), Some(&(6, "a")));
    /// assert_eq!(iter.next(), Some(&(6, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<L, R, A: Array<Item = (L, R)>> IntoIterator for SmallM2M<A> {
    type Item = (L, R);
    type IntoIter = smallvec::IntoIter<A>;

    /// Creates an iterator from a value.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let m2m: SmallM2M<[(u8, &str); 4]> = SmallM2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// let mut iter = m2m.into_iter();
    ///
    /// assert_eq!(iter.next(), Some((1, "a")));
    /// assert_eq!(iter.next(), Some((1, "b")));
    /// assert_eq!(iter.next(), Some((2, "a")));
    /// assert_eq!(iter.next(), Some((2, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<L, R, A: Array<Item = (L, R)>> SmallM2M<A> {
    /// Creates an empty SmallM2M.
    pub fn new() -> SmallM2M<A> {
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
    /// use p_m2m::SmallM2M;
    ///
    /// let mut m2m: SmallM2M<[(u8, &str); 5]> = SmallM2M::new();
    ///
    /// assert!(m2m.insert(1, "a"));
    /// assert!(m2m.insert(1, "b"));
    /// assert!(m2m.insert(2, "a"));
    /// assert!(m2m.insert(2, "b"));
    ///
    /// assert!(!m2m.insert(1, "a"));
    ///
    /// assert!(m2m.insert(1, "c"));
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(1, "a")));
    /// assert_eq!(iter.next(), Some(&(1, "b")));
    /// assert_eq!(iter.next(), Some(&(1, "c")));
    /// assert_eq!(iter.next(), Some(&(2, "a")));
    /// assert_eq!(iter.next(), Some(&(2, "b")));
    /// ```
    pub fn insert(&mut self, left: L, right: R) -> bool
    where
        (L, R): Ord,
    {
        let value = (left, right);

        if self.0.contains(&value) {
            return false;
        }

        self.0.push(value);
        self.0.sort();

        true
    }

    /// Returns the number of pairs in the m2m.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let mut m2m: SmallM2M<[(u8, &str); 0]> = SmallM2M::new();
    ///
    /// assert_eq!(m2m.len(), 0);
    /// m2m.insert(1, "a");
    /// assert_eq!(m2m.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the m2m contains no pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let mut m2m: SmallM2M<[(u8, &str); 0]> = SmallM2M::new();
    ///
    /// assert!(m2m.is_empty());
    /// m2m.insert(1, "a");
    /// assert!(!m2m.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Clears the m2m, removing all left-right pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let mut m2m: SmallM2M<[(u8, &str); 2]> = SmallM2M::from([(1, "a"), (1, "b")]);
    ///
    /// assert!(!m2m.is_empty());
    /// m2m.clear();
    /// assert!(m2m.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Removes some pairs from the m2m,
    /// returning the right values corresponding to the left if the left was previously in the m2m.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    /// use smallvec::smallvec;
    ///
    /// let mut m2m: SmallM2M<[(u8, &str); 2]> = SmallM2M::from([(1, "a"), (1, "b")]);
    ///
    /// assert_eq!(m2m.remove::<[&str; 2]>(&1), Some(smallvec!["a", "b"]));
    /// assert_eq!(m2m.remove::<[&str; 0]>(&1), None);
    ///
    /// assert!(m2m.is_empty());
    /// ```
    pub fn remove<T: Array<Item = R>>(&mut self, left: &L) -> Option<SmallVec<T>>
    where
        L: PartialEq,
    {
        let mut rights = SmallVec::new();

        let mut i = 0;
        while i < self.0.len() {
            if &self.0[i].0 == left {
                let (_, r) = self.0.remove(i);
                rights.push(r);
            } else {
                i += 1;
            }
        }

        if rights.is_empty() {
            return None;
        }

        Some(rights)
    }

    /// Returns `true` if the m2m contains the specified left-right pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let m2m: SmallM2M<[(u8, &str); 1]> = SmallM2M::from([(1, "a")]);
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
    /// use p_m2m::SmallM2M;
    ///
    /// let m2m: SmallM2M<[(u8, &str); 4]> = SmallM2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(1, "a")));
    /// assert_eq!(iter.next(), Some(&(1, "b")));
    /// assert_eq!(iter.next(), Some(&(2, "a")));
    /// assert_eq!(iter.next(), Some(&(2, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> slice::Iter<(L, R)> {
        self.0.iter()
    }

    /// Returns a mutable iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let mut m2m: SmallM2M<[(u8, &str); 4]> = SmallM2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// m2m.iter_mut().for_each(|(l, _)| *l *= 3);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(3, "a")));
    /// assert_eq!(iter.next(), Some(&(3, "b")));
    /// assert_eq!(iter.next(), Some(&(6, "a")));
    /// assert_eq!(iter.next(), Some(&(6, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> slice::IterMut<(L, R)> {
        self.0.iter_mut()
    }

    /// Extract a slice containing all pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::SmallM2M;
    ///
    /// let m2m: SmallM2M<[(u8, &str); 2]> = SmallM2M::from([(1, "a"), (1, "b")]);
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
