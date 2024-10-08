use crate::util::bytes_to_hex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Serialize, Deserialize)]
pub struct HashTreeNode<T> {
    pub value: T,
    pub hash: u64,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<T: PartialEq> PartialEq for HashTreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.hash == other.hash && self.parent == other.parent
    }
}

impl<T: Eq> Eq for HashTreeNode<T> {}

impl<T> HashTreeNode<T> {
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

impl<T: Debug> Debug for HashTreeNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} ({})",
            self.value,
            bytes_to_hex(&self.hash.to_le_bytes())
        )
    }
}
