#![allow(incomplete_features)]
#![feature(generic_associated_types)]
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

use map_trait::Map;

struct LastInsertMap<M, K, V> {
    inner_map: M,
    last_key: K,
    last_value: V,
}

impl<'m, K, V, M> LastInsertMap<M, K, V>
where
    K: Copy + Eq + Hash,
    V: 'm + Copy,
    M: Map<'m, K, V>,
{
    fn new(mut map: M, key: K, value: V) -> Self {
        map.insert(key, value);
        LastInsertMap {
            inner_map: map,
            last_key: key,
            last_value: value,
        }
    }

    fn get_last_insert(&self) -> (&K, &V) {
        (&self.last_key, &self.last_value)
    }
}

impl<'m, K, V, M> Map<'m, K, V> for LastInsertMap<M, K, V>
where
    K: Copy + Eq + Hash,
    V: 'm + Copy,
    M: Map<'m, K, V>,
{
    type GetGuard<'a> where 'm: 'a = M::GetGuard<'a>;

    #[inline]
    fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<Self::GetGuard<'a>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        self.inner_map.get(k)
    }

    fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.last_key = k;
        self.last_value = v;
        self.inner_map.insert(k, v)
    }
}

fn main() {
    let map = HashMap::new();
    let mut foo = LastInsertMap::new(map, 0, 1);
    assert_eq!(foo.get_last_insert(), (&0, &1));
    assert_eq!(foo.get(&0), Some(&1));
    assert_eq!(foo.insert(1, 2), None);
    assert_eq!(foo.get(&1), Some(&2));
    assert_eq!(foo.get_last_insert(), (&1, &2));
}
