use crate::util::bytes_to_hex;

use super::node::HashTreeNode;
use std::{
    fmt::{self, Debug, Formatter},
    hash::{DefaultHasher, Hash, Hasher},
};

pub struct HashTree<T: Hash + Debug> {
    values: Vec<Option<HashTreeNode<T>>>,
    root_id: Option<usize>,
    first_free_index: Option<usize>,
    should_compute_hashes: bool,
}

impl<T: Hash + Debug> Default for HashTree<T> {
    fn default() -> Self {
        Self {
            values: vec![],
            root_id: None,
            first_free_index: None,
            should_compute_hashes: true,
        }
    }
}

impl<T: Hash + Debug> HashTree<T> {
    pub fn compute_hash(&self, node: &HashTreeNode<T>) -> u64 {
        if !self.should_compute_hashes {
            return 0;
        }

        let mut hasher = DefaultHasher::new();
        node.value.hash(&mut hasher);
        for child in &node.children {
            let child = self.values[*child].as_ref().unwrap();
            child.hash.hash(&mut hasher);
        }
        hasher.finish()
    }

    pub fn refresh_hash_at(&mut self, id: usize) {
        if !self.should_compute_hashes {
            return;
        }

        let node = self.values[id].as_ref().unwrap();
        let hash = self.compute_hash(node);
        self.values[id].as_mut().unwrap().hash = hash;
    }

    pub fn insert(&mut self, parent: usize, value: T) -> Result<usize, ()> {
        // create node
        let mut node = HashTreeNode::new(value, Some(parent));
        let hash = self.compute_hash(&node);
        node.hash = hash;
        let parent = node.parent;

        // insert
        let index = self.insert_at_free_space(node);

        // update parent
        match parent
            .map_or(None, |parent| self.values.get_mut(parent))
            .map(|node| node.as_mut().unwrap())
        {
            Some(parent_node) => {
                parent_node.children.push(index);
            }
            None => return Err(()),
        }

        self.update_hashes_of_branch(parent);

        Ok(index)
    }

    fn insert_at_free_space(&mut self, node: HashTreeNode<T>) -> usize {
        let index = match self.first_free_index {
            Some(index) => {
                self.values[index] = Some(node);
                index
            }
            None => {
                self.values.push(Some(node));
                self.values.len() - 1
            }
        };

        self.first_free_index = self.values[index..]
            .as_ref()
            .iter()
            .position(|x| x.is_none());

        index
    }

    fn update_hashes_of_branch(&mut self, node: Option<usize>) {
        if !self.should_compute_hashes {
            return;
        }

        let mut current = node;
        while let Some(id) = current {
            self.refresh_hash_at(id);
            current = self.values[id].as_ref().unwrap().parent;
        }
    }

    pub fn insert_root(&mut self, value: T) -> usize {
        let node = HashTreeNode::new(value, None);
        let index = self.insert_at_free_space(node);
        self.root_id = Some(index);
        index
    }

    pub fn get_root(&self) -> Option<&HashTreeNode<T>> {
        self.root_id.map_or(None, |id| self.values[id].as_ref())
    }

    pub fn get_node(&self, id: usize) -> Option<&HashTreeNode<T>> {
        self.values[id].as_ref()
    }

    fn get_node_mut(&mut self, id: usize) -> Option<&mut HashTreeNode<T>> {
        self.values[id].as_mut()
    }

    pub fn set_value(&mut self, id: usize, value: T) -> Result<(), ()> {
        let slot = self.values.get_mut(id).ok_or(())?;
        let node = slot.as_mut().ok_or(())?;
        node.value = value;
        self.update_hashes_of_branch(Some(id));
        Ok(())
    }

    pub fn move_node(&mut self, id: usize, parent: usize) -> Result<(), ()> {
        if self.get_node(parent).is_none()
            || self.get_node(id).is_none()
            || self.get_node(id).unwrap().parent.is_none()
        {
            return Err(());
        }

        let new_parent = self.get_node_mut(parent).ok_or(())?;
        new_parent.children.push(id);

        let node = self.get_node_mut(id).ok_or(())?;
        let old_parent = node.parent;
        node.parent = Some(parent);

        if let Some(parent_node) = old_parent.map_or(None, |id| self.get_node_mut(id)) {
            parent_node.children.retain(|&child| child != id);
            self.update_hashes_of_branch(old_parent);
        }

        self.update_hashes_of_branch(Some(id));
        Ok(())
    }

    pub fn remove_node(&mut self, id: usize) -> Result<HashTreeNode<T>, ()> {
        if self.values[id].is_none() {
            return Err(());
        }

        if self.values.last().is_some() {
            self.values.push(None);
        }

        let node = self.free_slot(id)?;
        for child_id in &node.children {
            self.remove_nodes_rec(*child_id)?;
        }

        match node.parent {
            Some(parent_id) => {
                let parent_node = self.get_node_mut(parent_id).ok_or(())?;
                parent_node.children.retain(|&child| child != id);
                self.update_hashes_of_branch(node.parent);
            }
            None => {
                self.root_id = None;
            }
        }

        Ok(node)
    }
    
    fn remove_nodes_rec(&mut self, id: usize) -> Result<(), ()> {
        let node = self.free_slot(id)?;
        
        for child_id in node.children {
            self.remove_nodes_rec(child_id)?;
        }
        
        Ok(())
    }
    
    fn free_slot(&mut self, id: usize) -> Result<HashTreeNode<T>, ()> {
        let node = self.values.swap_remove(id).ok_or(())?;
        if self.first_free_index < Some(id) {
            self.first_free_index = Some(id);
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

        self.pretty_print_rec(root_id, "".to_string(), "".to_string(), false, true);
    }

    fn pretty_print_rec(
        &self,
        id: usize,
        indent: String,
        prefix: String,
        last_flag: bool,
        top_level: bool,
    ) {
        let node = self.values[id].as_ref().unwrap();
        println!("{}{}{:?}", indent, prefix, node);

        if node.children.is_empty() {
            return;
        }

        let last = node.children.len() - 1;
        for (index, child) in node.children.iter().enumerate() {
            let prefix = if index == last {
                "┗━━━"
            } else {
                "┣━━━"
            };

            let indent = if top_level {
                indent.clone()
            } else if last_flag {
                indent.clone() + "    "
            } else {
                indent.clone() + "┃   "
            };

            self.pretty_print_rec(*child, indent, prefix.to_string(), index == last, false);
        }
    }

    pub fn refresh_hashes(&mut self) {
        if !self.should_compute_hashes {
            return;
        }

        let root_id = match self.root_id {
            Some(id) => id,
            None => return,
        };

        let mut visited = vec![false; self.values.len()];
        let mut stack: Vec<usize> = Vec::new();

        stack.push(root_id);

        while let Some(node_idx) = stack.pop() {
            if visited[node_idx] {
                self.refresh_hash_at(node_idx);
            } else {
                visited[node_idx] = true;
                stack.push(node_idx);

                let node = self.values[node_idx].as_ref().unwrap();
                for &child_id in &node.children {
                    if !visited[child_id] {
                        stack.push(child_id);
                    }
                }
            }
        }
    }

    pub fn should_compute_hashes(&self) -> bool {
        self.should_compute_hashes
    }

    pub fn set_should_compute_hashes(&mut self, should_compute_hashes: bool) {
        self.should_compute_hashes = should_compute_hashes;
        if should_compute_hashes {
            self.refresh_hashes();
        }
    }
}

impl<T: Hash + Debug> Debug for HashTreeNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self.value, bytes_to_hex(&self.hash.to_le_bytes()))
    }
}

#[cfg(test)]
mod test {
    use super::HashTree;

    #[test]
    fn it_works() {
        let mut tree = HashTree::default();
        tree.set_should_compute_hashes(false);

        let root_id = tree.insert_root("root");

        let child_1_id = tree.insert(root_id, "child1").unwrap();
        let child_2_id = tree.insert(root_id, "child2").unwrap();
        let child_1_1_id = tree.insert(child_1_id, "child3").unwrap();
        let child_1_2_id = tree.insert(child_1_id, "child4").unwrap();
        let child_1_2_1_id = tree.insert(child_1_2_id, "child5").unwrap();
        let child_1_2_1_1_id = tree.insert(child_1_2_1_id, "child6").unwrap();
        let child_1_2_1_1_1_id = tree.insert(child_1_2_1_1_id, "child7").unwrap();
        let child_1_2_id = tree.insert(child_1_id, "child8").unwrap();
        let child_2_1_id = tree.insert(child_2_id, "child9").unwrap();
        let child_2_2_id = tree.insert(child_2_id, "child10").unwrap();
        let child_2_2_1_id = tree.insert(child_2_2_id, "child11").unwrap();
        let child_2_2_2_id = tree.insert(child_2_2_id, "child12").unwrap();
        let child_2_2_3_id = tree.insert(child_2_2_id, "child13").unwrap();

        let mut parent = child_1_1_id;

        tree.set_should_compute_hashes(true);

        tree.pretty_print();
        tree.move_node(child_2_2_id, child_1_id).unwrap();
        tree.pretty_print();
    }
}
