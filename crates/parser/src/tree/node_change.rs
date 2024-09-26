use indextree::NodeId;
use kern::Node;

type ParentNodeId = NodeId;

#[derive(Debug, Clone)]
pub enum NodeChange<'a> {
    Update(NodeId, &'a Node),
    Addition(NodeId, ParentNodeId),
    Deletion(NodeId),
}

impl NodeChange<'_> {
    pub fn node_id(&self) -> NodeId {
        match self {
            NodeChange::Update(node_id, _) => *node_id,
            NodeChange::Addition(node_id, _) => *node_id,
            NodeChange::Deletion(node_id) => *node_id,
        }
    }
}