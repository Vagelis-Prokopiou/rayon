//! This module contains the parallel iterator types for heaps
//! (`BinaryHeap<T>`). You will rarely need to interact with it directly
//! unless you have need to name one of the iterator types.

use std::collections::BinaryHeap;

use iter::*;
use iter::internal::*;

use vec;

/// Parallel iterator over a binary heap
#[derive(Debug, Clone)]
pub struct IntoIter<T: Ord + Send> {
    inner: vec::IntoIter<T>,
}

impl<T: Ord + Send> IntoParallelIterator for BinaryHeap<T> {
    type Item = T;
    type Iter = IntoIter<T>;

    fn into_par_iter(self) -> Self::Iter {
        IntoIter { inner: Vec::from(self).into_par_iter() }
    }
}

delegate_indexed_iterator!{
    IntoIter<T> => T,
    impl<T: Ord + Send>
}


/// Parallel iterator over an immutable reference to a binary heap
#[derive(Debug, Clone)]
pub struct Iter<'a, T: Ord + Sync + 'a> {
    inner: vec::IntoIter<&'a T>,
}

into_par_vec!{
    &'a BinaryHeap<T> => Iter<'a, T>,
    impl<'a, T: Ord + Sync>
}

delegate_indexed_iterator!{
    Iter<'a, T> => &'a T,
    impl<'a, T: Ord + Sync + 'a>
}


// `BinaryHeap` doesn't have a mutable `Iterator`
