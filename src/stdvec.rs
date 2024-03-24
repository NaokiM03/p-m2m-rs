use core::fmt::{self, Debug};
use core::iter::{FromIterator, IntoIterator};
use core::slice;

use std::vec;

/// A many-to-many implemented as a `Vec<(L, R)>`.
///
/// M2M is just a wrapper around a Vec.
pub struct M2M<L, R>(Vec<(L, R)>);

impl<L, R> Debug for M2M<L, R>
where
    (L, R): Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.0.iter()).finish()
    }
}

impl<L, R> Default for M2M<L, R> {
    /// Creates an empty `M2M<L, R>`.
    #[inline]
    fn default() -> Self {
        M2M(Vec::new())
    }
}

impl<L, R> FromIterator<(L, R)> for M2M<L, R>
where
    (L, R): Ord,
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
    (L, R): Ord,
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

impl<'a, L, R> IntoIterator for &'a M2M<L, R> {
    type Item = &'a (L, R);
    type IntoIter = slice::Iter<'a, (L, R)>;

    /// Creates an iterator from a value.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
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

impl<'a, L, R> IntoIterator for &'a mut M2M<L, R> {
    type Item = &'a mut (L, R);
    type IntoIter = slice::IterMut<'a, (L, R)>;

    /// Creates an iterator from a value.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = &mut M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
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

impl<L, R> IntoIterator for M2M<L, R> {
    type Item = (L, R);
    type IntoIter = vec::IntoIter<(L, R)>;

    /// Creates an iterator from a value.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
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

impl<L, R> M2M<L, R> {
    /// Creates an empty M2M.
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

    /// Removes some pairs from the m2m,
    /// returning the right values corresponding to the left if the left was previously in the m2m.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b")]);
    ///
    /// assert_eq!(m2m.remove(&1), Some(vec!["a", "b"]));
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
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a")]);
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
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
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
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// m2m.iter_mut().for_each(|(l, _)| *l += 2);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(3, "a")));
    /// assert_eq!(iter.next(), Some(&(3, "b")));
    /// assert_eq!(iter.next(), Some(&(4, "a")));
    /// assert_eq!(iter.next(), Some(&(4, "b")));
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

    /// Extract a mutable slice containing all pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b")]);
    ///
    /// let slice = m2m.as_mut_slice();
    ///
    /// assert_eq!(slice[0], (1, "a"));
    /// assert_eq!(slice[1], (1, "b"));
    ///
    /// slice[1].0 = 3;
    ///
    /// assert_eq!(slice[1], (3, "b"));
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [(L, R)] {
        self.0.as_mut_slice()
    }

    /// Retains only the pairs specified by the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// m2m.retain(|(l, _)| l % 2 == 0);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(2, "a")));
    /// assert_eq!(iter.next(), Some(&(2, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&(L, R)) -> bool,
    {
        self.0.retain_mut(|pair| f(pair));
    }

    /// Rejects the pairs specified by the predicate.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// m2m.reject(|(l, _)| l % 2 == 0);
    ///
    /// let mut iter = m2m.iter();
    ///
    /// assert_eq!(iter.next(), Some(&(1, "a")));
    /// assert_eq!(iter.next(), Some(&(1, "b")));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn reject<F>(&mut self, mut f: F)
    where
        F: FnMut(&(L, R)) -> bool,
    {
        self.0.retain_mut(|pair| !f(pair));
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
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "c"), (2, "d")]);
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
            return None;
        }

        Some(rights)
    }

    /// Returns a reference to the left values corresponding to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (2, "b"), (3, "a"), (4, "b")]);
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
            return None;
        }

        Some(lefts)
    }

    /// Returns a mutable reference to the right values corresponding to the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, 11), (1, 111), (2, 22), (2, 222)]);
    ///
    /// let rights = m2m.get_rights_mut(&1).unwrap();
    /// rights.into_iter().for_each(|r| *r *= 3);
    ///
    /// assert_eq!(m2m.rights(), Some(vec![&22, &33, &222, &333]));
    /// ```
    pub fn get_rights_mut(&mut self, left: &L) -> Option<Vec<&mut R>>
    where
        L: PartialEq,
    {
        let rights: Vec<&mut R> = self
            .0
            .iter_mut()
            .filter(|(l, _)| l == left)
            .map(|(_, r)| r)
            .collect();

        if rights.is_empty() {
            return None;
        }

        Some(rights)
    }

    /// Returns a mutable reference to the left values corresponding to the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let mut m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b")]);
    ///
    /// let lefts = m2m.get_lefts_mut(&"a").unwrap();
    /// lefts.into_iter().for_each(|l| *l *= 3);
    ///
    /// assert_eq!(m2m.lefts(), Some(vec![&1, &2, &3, &6]));
    /// ```
    pub fn get_lefts_mut(&mut self, right: &R) -> Option<Vec<&mut L>>
    where
        R: PartialEq,
    {
        let lefts: Vec<&mut L> = self
            .0
            .iter_mut()
            .filter(|(_, r)| r == right)
            .map(|(l, _)| l)
            .collect();

        if lefts.is_empty() {
            return None;
        }

        Some(lefts)
    }

    /// Returns `true` if the m2m contains the specified left value.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b")]);
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
    /// let m2m = M2M::from([(1, "a"), (1, "b")]);
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

    /// Returns a reference to all left values.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (2, "b"), (3, "a"), (4, "b"), (1, "a")]);
    ///
    /// let lefts = m2m.lefts();
    /// assert_eq!(lefts, Some(vec![&1, &2, &3, &4]));
    /// ```
    pub fn lefts(&self) -> Option<Vec<&L>>
    where
        L: Ord,
    {
        let mut v: Vec<&L> = self.0.iter().map(|(l, _)| l).collect();

        if v.is_empty() {
            return None;
        }

        v.sort();
        v.dedup();

        Some(v)
    }

    /// Returns a reference to all right values.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "c"), (2, "d"), (1, "a")]);
    ///
    /// let rights = m2m.rights();
    /// assert_eq!(rights, Some(vec![&"a", &"b", &"c", &"d"]));
    /// ```
    pub fn rights(&self) -> Option<Vec<&R>>
    where
        R: Ord,
    {
        let mut v: Vec<&R> = self.0.iter().map(|(_, r)| r).collect();

        if v.is_empty() {
            return None;
        }

        v.sort();
        v.dedup();

        Some(v)
    }

    /// Returns all left values.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (2, "b"), (3, "a"), (4, "b"), (1, "a")]);
    ///
    /// let lefts = m2m.into_lefts();
    /// assert_eq!(lefts, Some(vec![1, 2, 3, 4]));
    /// ```
    pub fn into_lefts(self) -> Option<Vec<L>>
    where
        L: Ord,
    {
        let mut v: Vec<L> = self.0.into_iter().map(|(l, _)| l).collect();

        if v.is_empty() {
            return None;
        }

        v.sort();
        v.dedup();

        Some(v)
    }

    /// Returns all right values.
    /// The m2m cannot be used after calling this.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "c"), (2, "d"), (1, "a")]);
    ///
    /// let rights = m2m.into_rights();
    /// assert_eq!(rights, Some(vec!["a", "b", "c", "d"]));
    /// ```
    pub fn into_rights(self) -> Option<Vec<R>>
    where
        R: Ord,
    {
        let mut v: Vec<R> = self.0.into_iter().map(|(_, r)| r).collect();

        if v.is_empty() {
            return None;
        }

        v.sort();
        v.dedup();

        Some(v)
    }

    /// Flips left an right in all pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use p_m2m::M2M;
    ///
    /// let m2m = M2M::from([(1, "a"), (1, "b"), (2, "a"), (2, "b"), (3, "a")]);
    ///
    /// let flipped = m2m.flip();
    ///
    /// let rights = flipped.get_rights(&"a");
    /// assert_eq!(rights, Some(vec![&1, &2, &3]));
    /// ```
    pub fn flip(&self) -> M2M<R, L>
    where
        (L, R): Clone,
        (R, L): Ord,
    {
        let mut v: Vec<(R, L)> = self.0.iter().cloned().map(|(l, r)| (r, l)).collect();

        v.sort();

        M2M(v)
    }
}
