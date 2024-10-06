use crate::util::bytes_to_hex;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct HashTreeNode<T> {
    pub value: T,
    pub hash: u64,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

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
