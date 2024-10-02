use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
    hash::{DefaultHasher, Hash, Hasher},
};

pub struct Node<T: Hash + Debug> {
    pub value: T,
    pub hash: u64,
    pub parent: Option<u64>,
    pub children: Vec<u64>,
}

impl<T: Hash + Debug> Node<T> {
    pub fn new(value: T, parent: Option<u64>) -> Self {
        let children = vec![];
        let hash = Self::get_hash(&value, &children);

        Self {
            parent,
            value,
            hash,
            children,
        }
    }

    pub fn set_children(&mut self, children: Vec<u64>) {
        self.children = children;
        self.hash = Self::get_hash(&self.value, &self.children);
    }

    pub fn get_hash(value: &T, children: &[u64]) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        for child in children {
            child.hash(&mut hasher);
        }
        hasher.finish()
    }
}

pub struct TitHashTree<T: Hash + Debug> {
    pub values: HashMap<u64, Node<T>>,
    pub root_id: Option<u64>,
}

impl<T: Hash + Debug> Default for TitHashTree<T> {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
            root_id: None,
        }
    }
}

impl<T: Hash + Debug> TitHashTree<T> {
    pub fn insert(&mut self, parent: u64, value: T) -> Result<u64, ()> {
        let node = Node::new(value, Some(parent));

        self.insert_node(node)
    }

    fn insert_node(&mut self, node: Node<T>) -> Result<u64, ()> {
        let hash = node.hash;
        let parent = node.parent;
        self.values.insert(hash, node);

        match parent.map_or(None, |parent| self.values.get_mut(&parent)) {
            Some(parent_node) => {
                parent_node.children.push(hash);
            }
            None => return Err(()),
        }

        self.update_hashes_of_branch(parent);

        Ok(hash)
    }

    fn update_hashes_of_branch(&mut self, node: Option<u64>) {
        let mut current = node;
        while let Some(id) = current {
            let mut node = self.values.remove(&id).unwrap();
            node.hash = Node::get_hash(&node.value, &node.children);
            current = node.parent;
            self.values.insert(id, node);
        }
    }

    pub fn insert_root(&mut self, value: T) -> u64 {
        let node = Node::new(value, None);
        let hash = node.hash;
        self.values.insert(hash, node);
        self.root_id = Some(hash);

        hash
    }

    pub fn get_root(&self) -> Option<&Node<T>> {
        self.root_id.map_or(None, |id| self.values.get(&id))
    }

    pub fn get_node(&self, id: u64) -> Option<&Node<T>> {
        self.values.get(&id)
    }

    pub fn set_value(&mut self, id: u64, value: T) -> Result<(), ()> {
        let node = self.values.get_mut(&id).ok_or(())?;
        node.value = value;
        self.update_hashes_of_branch(Some(id));
        Ok(())
    }

    pub fn set_parent(&mut self, id: u64, parent: u64) -> Result<(), ()> {
        let new_parent = self.values.get_mut(&parent).ok_or(())?;
        new_parent.children.push(id);

        let node = self.values.get_mut(&id).ok_or(())?;
        let old_parent = node.parent;
        node.parent = Some(parent);

        match old_parent.map_or(None, |id| self.values.get_mut(&id)) {
            Some(parent_node) => {
                parent_node.children.retain(|&child| child != id);
                self.update_hashes_of_branch(old_parent);
            }
            None => {}
        }

        self.update_hashes_of_branch(Some(id));
        Ok(())
    }

    pub fn remove_node(&mut self, id: u64) -> Result<Node<T>, ()> {
        let node = self.values.remove(&id).ok_or(())?;

        match node.parent {
            Some(parent_node) => {
                let parent_node = self.values.get_mut(&parent_node).ok_or(())?;
                parent_node.children.retain(|&child| child != id);
                self.update_hashes_of_branch(node.parent);
            }
            None => {
                self.root_id = None;
            }
        }

        Ok(node)
    }

    pub fn pretty_print(&self) {
        let root_id = match self.root_id {
            Some(id) => id,
            None => {
                println!("Empty tree");
                return;
            }
        };

        self.pretty_print_rec(root_id, 0);
    }

    pub fn pretty_print_rec(&self, id: u64, indent: usize) {
        let node = self.values.get(&id).unwrap();
        println!("{:indent$}>{:?}", "", node, indent = indent);

        for child in &node.children {
            self.pretty_print_rec(*child, indent + 2);
        }
    }
}

impl<T: Hash + Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[cfg(test)]
mod test {
    use super::TitHashTree;

    #[test]
    fn it_works() {
        let mut tree = TitHashTree::default();

        let root_id = tree.insert_root("root");

        let child_1_id = tree.insert(root_id, "child1").unwrap();
        let child_2_id = tree.insert(root_id, "child2").unwrap();
        let child_1_1_id = tree.insert(child_1_id, "child3").unwrap();
        let child_1_2_id = tree.insert(child_1_id, "child4").unwrap();
        let child_2_1_id = tree.insert(child_2_id, "child5").unwrap();

        tree.pretty_print();

        tree.set_parent(child_2_1_id, child_1_id).unwrap();
        tree.pretty_print();
    }
}
