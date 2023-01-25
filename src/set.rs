use std::borrow::Borrow;
use std::hash::Hash;

/// A generic Set trait
///
/// # Examples
///
/// This is a toy example of a set which reexposes an inner set and stores the
/// most recent value to be inserted. Because the LastInsertSet implements
/// Set, it can be seamlessly used as a replacement for other sets.
/// ```
/// use std::borrow::Borrow;
/// use std::hash::Hash;
/// use std::collections::HashSet;
///
/// use map_trait::set::Set;
/// struct LastInsertSet<T, S> {
///     inner_set: S,
///     last_value: T,
/// }
///
/// impl<T, S> LastInsertSet<T, S>
/// where
///     T: Copy,
///     S: Set<T>
/// {
///     fn new(mut set: S, value: T) -> Self {
///         set.insert(value);
///         LastInsertSet {
///             inner_set: set,
///             last_value: value,
///         }
///     }
///
///     fn get_last_insert(&self) -> &T {
///         &self.last_value
///     }
/// }
///
/// impl<T, S> Set<T> for LastInsertSet<T, S>
/// where
///     T: Copy,
///     S: Set<T>
/// {
///
///     #[inline]
///     fn contains<Q: ?Sized>(&self, value: &Q) -> bool
///     where
///         T: Borrow<Q>,
///         Q: Hash + Eq + Ord
///     {
///         self.inner_set.contains(value)
///     }
///
///     fn insert(&mut self, value: T) -> bool
///     {
///         self.last_value = value;
///         self.inner_set.insert(value)
///     }
///
/// }
///
/// # fn main() {
///     let mut set = LastInsertSet::new(HashSet::new(), 0);
///     assert_eq!(set.get_last_insert(), &0);
///     assert!(set.contains(&0));
///     assert!(set.insert(1));
///     assert!(set.contains(&1));
///     assert_eq!(set.get_last_insert(), &1);
/// # }
/// ```

pub trait Set<T> {
    fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + Ord;

    fn insert(&mut self, value: T) -> bool;
}

impl<T, S> Set<T> for std::collections::HashSet<T, S>
where
    T: Hash + Eq,
    S: std::hash::BuildHasher,
{
    #[inline]
    fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        std::collections::HashSet::contains(self, value)
    }

    #[inline]
    fn insert(&mut self, value: T) -> bool {
        std::collections::HashSet::insert(self, value)
    }
}

impl<T> Set<T> for std::collections::BTreeSet<T>
where
    T: Ord,
{
    #[inline]
    fn contains<Q: ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        std::collections::BTreeSet::contains(self, value)
    }

    #[inline]
    fn insert(&mut self, value: T) -> bool {
        std::collections::BTreeSet::insert(self, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_set_contains<T>(set: &impl Set<T>, value: T)
    where
        T: Hash + Eq + Ord,
    {
        assert!(set.contains(&value));
    }

    fn assert_set_insert<T>(set: &mut impl Set<T>, value: T)
    where
        T: Hash + Eq + Ord,
    {
        assert!(set.insert(value));
    }

    #[test]
    fn test_hash_set() {
        let mut set = std::collections::HashSet::new();

        assert_set_insert(&mut set, 1);
        assert_set_contains(&set, 1);
    }

    #[test]
    fn test_btree_set() {
        let mut set = std::collections::BTreeSet::new();

        assert_set_insert(&mut set, 1);
        assert_set_contains(&set, 1);
    }
}
