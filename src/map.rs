use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Deref;

/// A generic Map trait
///
/// # Examples
///
/// This is a toy example of a map which reexposes an inner map and stores the
/// most recent key and value to be inserted. Because the LastInsertMap implements
/// Map, it can be seamlessly used as a replacement for other maps.
/// ```
/// use std::borrow::Borrow;
/// use std::hash::Hash;
/// use std::collections::HashMap;
///
/// use map_trait::map::Map;
/// struct LastInsertMap<M, K, V> {
///     inner_map: M,
///     last_key: K,
///     last_value: V,
/// }
///
/// impl<'m, K, V, M> LastInsertMap<M, K, V>
/// where
///     K: Copy,
///     V: 'm + Copy,
///     M: Map<'m, K, V>,
/// {
///     fn new(mut map: M, key: K, value: V) -> Self {
///         map.insert(key, value);
///         LastInsertMap {
///             inner_map: map,
///             last_key: key,
///             last_value: value,
///         }
///     }
///
///     fn get_last_insert(&self) -> (&K, &V) {
///         (&self.last_key, &self.last_value)
///     }
/// }
///
/// impl<'m, K, V, M> Map<'m, K, V> for LastInsertMap<M, K, V>
/// where
///     K: Copy,
///     V: 'm + Copy,
///     M: Map<'m, K, V>,
/// {
///     type GetGuard<'a> = M::GetGuard<'a> where Self: 'a;
///
///     #[inline]
///     fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<Self::GetGuard<'a>>
///     where
///         K: Borrow<Q>,
///         Q: Hash + Eq + Ord,
///     {
///         self.inner_map.get(k)
///     }
///
///     #[inline]
///     fn insert(&mut self, k: K, v: V) -> Option<V> {
///         self.last_key = k;
///         self.last_value = v;
///         self.inner_map.insert(k, v)
///     }
/// }
///
/// # fn main() {
///     let mut map = LastInsertMap::new(HashMap::new(), 0, 1);
///     assert_eq!(map.get_last_insert(), (&0, &1));
///     assert_eq!(map.get(&0), Some(&1));
///     assert_eq!(map.insert(1, 2), None);
///     assert_eq!(map.get(&1), Some(&2));
///     assert_eq!(map.get_last_insert(), (&1, &2));
/// # }
/// ```
pub trait Map<'m, K, V: 'm> {
    type GetGuard<'a>: Deref<Target = V>
    where
        Self: 'a;

    fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<Self::GetGuard<'a>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord;
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}

impl<'m, K, V, S> Map<'m, K, V> for std::collections::HashMap<K, V, S>
where
    K: Hash + Eq,
    V: 'm,
    S: std::hash::BuildHasher,
{
    type GetGuard<'a> = &'a V where Self: 'a;

    #[inline]
    fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<Self::GetGuard<'a>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        std::collections::HashMap::get(self, k)
    }

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        std::collections::HashMap::insert(self, k, v)
    }
}

impl<'m, K, V> Map<'m, K, V> for std::collections::BTreeMap<K, V>
where
    K: Ord,
    V: 'm,
{
    type GetGuard<'a> = &'a V where Self: 'a;

    #[inline]
    fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<Self::GetGuard<'a>>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        std::collections::BTreeMap::get(self, k)
    }

    #[inline]
    fn insert(&mut self, k: K, v: V) -> Option<V> {
        std::collections::BTreeMap::insert(self, k, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Debug;

    fn assert_map_get<'m, K, V>(map: &impl Map<'m, K, V>, k: K, v: V)
    where
        K: Hash + Eq + Ord,
        V: 'm + Clone + Eq + Debug,
    {
        assert_eq!(map.get(&k).unwrap().clone(), v);
    }

    fn assert_map_insert<'m, K, V>(map: &mut impl Map<'m, K, V>, k: K, v: V, o: Option<V>)
    where
        K: Hash + Eq,
        V: 'm + Eq + Debug,
    {
        assert_eq!(map.insert(k, v), o);
    }

    #[test]
    fn test_hash_map() {
        let mut map = std::collections::HashMap::new();

        assert_map_insert(&mut map, 1, 2, None);
        assert_map_get(&map, 1, 2);
    }

    #[test]
    fn test_btree_map() {
        let mut map = std::collections::BTreeMap::new();

        assert_map_insert(&mut map, 1, 2, None);
        assert_map_get(&map, 1, 2);
    }
}
