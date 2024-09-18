use indextree::NodeId;

#[derive(Debug, Clone)]
pub enum MutableNodeChangeKind<'a> {
    KindUpdate { kind: &'a String, value: &'a Option<String> },
    ValueUpdate(&'a String),
    Addition { parent: NodeId },
    Deletion,
}

#[derive(Debug, Clone)]
pub struct MutableNodeChange<'a> {
    pub node: NodeId,
    pub kind: MutableNodeChangeKind<'a>,
}