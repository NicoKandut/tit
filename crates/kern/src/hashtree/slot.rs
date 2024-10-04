use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Slot<T> {
    Empty {
        previous: Option<usize>,
        next: Option<usize>,
    },
    Filled {
        item: T,
    },
}

impl<T> Slot<T> {
    pub fn new_empty(previous: Option<usize>, next: Option<usize>) -> Self {
        Slot::Empty { previous, next }
    }

    pub fn new_filled(value: T) -> Self {
        Slot::Filled { item: value }
    }
}
