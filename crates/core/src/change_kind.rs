use bincode::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone, Copy, Hash)]
pub enum ChangeKind {
    ADDITION = 0,
    DELETION = 1,
    UPDATE = 2,
}