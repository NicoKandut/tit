use std::fmt::Debug;
use std::hash::Hash;

pub struct HashTreeNode<T: Hash + Debug> {
    pub value: T,
    pub hash: u64,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T: Hash + Debug> HashTreeNode<T> {
    pub fn new(value: T, parent: Option<usize>) -> Self {
        let children = vec![];

        Self {
            parent,
            value,
            hash: 0,
            children,
        }
    }
}
