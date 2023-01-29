use std::borrow::Borrow;
use std::hash::Hash;
use std::ops::Deref;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Poll, Context};

use crate::map::Map;

pub trait AsyncMap<'m, K, V: 'm> {
    type GetGuard<'a>: Deref<Target = V> where 'm: 'a;
    type GetFuture<'a, Q>: Future<Output = Option<Self::GetGuard<'a>>>;
    type InsertFuture<'a>: Future<Output = Option<V>>;


    fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Self::GetFuture<'a, Q>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord;
    fn insert<'a>(&'m mut self, k: K, v: V) -> Self::InsertFuture<'a>;
}

pub mod futures {
    use super::*;

    pub struct SyncGetFuture<'a, 'm: 'a, K: Borrow<Q> + Hash + Eq + Ord, Q: ?Sized + Hash + Eq + Ord, V: 'm, M: Map<'m, K, V>> {
        map: &'m M,
        key: &'a Q,
        _key: PhantomData<K>,
        _value: PhantomData<V>,
    }
    
    impl<'a, 'm: 'a, K, Q, V: 'm, M> SyncGetFuture<'a, 'm, K, Q, V, M>
    where
        M: Map<'m, K, V>,
        K: Borrow<Q> + Hash + Eq + Ord,
        Q: ?Sized + Hash + Eq + Ord,
    {
        pub fn new(map: &'m M, key: &'m Q) -> Self {
            SyncGetFuture {
                map,
                key,
                _key: PhantomData,
                _value: PhantomData,
            }
        }
    }
    
    impl<'a, 'm: 'a, K, Q, V: 'm, M> Future for SyncGetFuture<'a, 'm, K, Q, V, M>
    where
        M: Map<'m, K, V>,
        K: Borrow<Q> + Hash + Eq + Ord,
        Q: ?Sized + Hash + Eq + Ord,
    {
        type Output = Option<M::GetGuard<'a>>;
    
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(self.map.get(self.key))
        }
    }
    
    pub struct SyncInsertFuture<'a, 'm: 'a, K: Hash + Eq + Ord, V: 'm, M: Map<'m, K, V>> {
        map: &'m mut M,
        key: K,
        value: V,
        _future_lifetime: PhantomData<&'a ()>,
    }

    impl<'a, 'm: 'a, K, V: 'm, M> SyncInsertFuture<'a, 'm, K, V, M>
    where
        M: Map<'m, K, V>,
        K: Hash + Eq + Ord,
    {
        pub fn new(map: &'m mut M, key: K, value: V) -> Self {
            SyncInsertFuture {
                map,
                key,
                value,
                _future_lifetime: PhantomData
            }
        }
    }
    
    impl<'a, 'm: 'a, K, V: 'm, M> Future for SyncInsertFuture<'a, 'm, K, V, M>
    where
        M: Map<'m, K, V>,
        K: Hash + Eq + Ord,
    {
        type Output = Option<V>;
    
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            Poll::Ready(self.map.insert(self.key, self.value))
        }
    }
}

impl<'m, K, V, M> AsyncMap<'m, K, V> for M
where
    K: 'm + Hash + Eq + Ord,
    V: 'm,
    M: 'm + Map<'m, K, V>
{
    type GetGuard<'a> where 'm: 'a = &'a V;
    type GetFuture<'a, Q> where 'm: 'a, K: Borrow<Q>, Q: 'a + ?Sized + Hash + Eq + Ord = futures::SyncGetFuture<'a, 'm, K, Q, V, Self>;
    type InsertFuture<'a> where 'm: 'a = futures::SyncInsertFuture<'a, 'm, K, V, M>;


    #[inline]
    fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Self::GetFuture<'a, Q>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + Ord,
    {
        futures::SyncGetFuture::new(&self, k)
    }

    #[inline]
    fn insert<'a>(&'m mut self, k: K, v: V) -> Self::InsertFuture<'a> {
        futures::SyncInsertFuture::new(self, k, v)
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
