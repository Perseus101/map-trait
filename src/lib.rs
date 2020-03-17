#![feature(generic_associated_types, associated_type_defaults)]

use std::borrow::Borrow;
use std::ops::Deref;

pub trait MapGuard<'a, T>: Deref + std::marker::Sized {}

impl<'a, T> MapGuard<'a, T> for &'a T where T: 'a {}

pub trait Map<K, V> {
    type GetGuard<'a>: MapGuard<'a, V>;

    fn get<'a, Q: ?Sized>(&'a self, k: &Q) -> Option<Self::GetGuard<'a>> where K: Borrow<Q>;
    fn insert(&mut self, k: K, v: V) -> Option<V>;
}