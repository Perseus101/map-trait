#![allow(incomplete_features)]
#![feature(generic_associated_types)]

use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Deref;

pub trait MapGuard<'a, T>: Deref<Target = T> + std::marker::Sized {}

impl<'a, T> MapGuard<'a, T> for &'a T {}

pub trait Map<'m, K, V: 'm> {
    type GetGuard<'a>: MapGuard<'a, V> where 'm: 'a;

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
    type GetGuard<'a> where 'm: 'a = &'a V;

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
    type GetGuard<'a> where 'm: 'a = &'a V;

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
