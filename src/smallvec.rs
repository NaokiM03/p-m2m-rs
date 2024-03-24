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
}

impl<L, R, A: Array<Item = (L, R)>> SmallM2M<A> {
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
}
