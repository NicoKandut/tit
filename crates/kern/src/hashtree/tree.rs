use super::{node::HashTreeNode, slot::Slot};
use crate::{util::BinaryFile, Change, Node};
use core::panic;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Serialize, Deserialize)]
pub struct HashTree<T> {
    values: Vec<Slot<HashTreeNode<T>>>,
    root_id: Option<usize>,
    first_free_index: Option<usize>,
    should_compute_hashes: bool,
}

impl<T> BinaryFile for HashTree<T> {}

impl<T> Default for HashTree<T> {
    fn default() -> Self {
        Self {
            values: vec![],
            root_id: None,
            first_free_index: None,
            should_compute_hashes: true,
        }
    }
}

impl<T: Hash> HashTree<T> {
    pub fn compute_hash(&self, node: &HashTreeNode<T>) -> u64 {
        if !self.should_compute_hashes {
            return 0;
        }

        let mut hasher = DefaultHasher::new();
        node.value.hash(&mut hasher);
        for child in &node.children {
            match self.values.get(*child) {
                Some(Slot::Filled { item: child }) => {
                    child.hash.hash(&mut hasher);
                }
                _ => panic!("Child node not found"),
            }
        }
        hasher.finish()
    }

    pub fn refresh_hash_at(&mut self, id: usize) {
        if !self.should_compute_hashes {
            return;
        }

        let node = match self.values.get(id) {
            Some(Slot::Filled { item }) => item,
            _ => return,
        };

        let hash = self.compute_hash(node);

        match self.values.get_mut(id) {
            Some(Slot::Filled { item }) => item.hash = hash,
            _ => return,
        };
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
            .map_or(None, |s| match s {
                Slot::Filled { item } => Some(item),
                _ => None,
            }) {
            Some(parent_node) => {
                parent_node.children.push(index);
            }
            None => return Err(()),
        }

        self.update_hashes_of_branch(parent);

        Ok(index)
    }

    fn insert_at_free_space(&mut self, node: HashTreeNode<T>) -> usize {
        // find index or insert new
        let index = match self.first_free_index {
            Some(index) => index,
            None => {
                self.values.push(Slot::new_empty(None, None));
                self.values.len() - 1
            }
        };

        // fill slot
        let slot = self.values.get_mut(index).unwrap();
        let next_free = match slot {
            Slot::Empty { next, .. } => next.clone(),
            _ => panic!("Slot is not empty"),
        };
        *slot = Slot::new_filled(node);

        // update first free index
        self.first_free_index = next_free;

        index
    }

    fn update_hashes_of_branch(&mut self, node: Option<usize>) {
        if !self.should_compute_hashes {
            return;
        }

        let mut current = node;
        while let Some(id) = current {
            self.refresh_hash_at(id);
            let node = match self.values.get_mut(id).unwrap() {
                Slot::Filled { item } => item,
                _ => panic!("Node not found"),
            };
            current = node.parent;
        }
    }

    pub fn insert_root(&mut self, value: T) -> usize {
        let node = HashTreeNode::new(value, None);
        let index = self.insert_at_free_space(node);
        self.root_id = Some(index);
        index
    }

    pub fn get_root(&self) -> Option<&HashTreeNode<T>> {
        self.root_id
            .map_or(None, |id| self.values.get(id))
            .map_or(None, |s| match s {
                Slot::Filled { item } => Some(item),
                _ => None,
            })
    }

    pub fn get_node(&self, id: usize) -> Option<&HashTreeNode<T>> {
        match self.values.get(id) {
            Some(Slot::Filled { item }) => Some(item),
            _ => None,
        }
    }

    fn get_node_mut(&mut self, id: usize) -> Option<&mut HashTreeNode<T>> {
        self.values.get_mut(id).map_or(None, |s| match s {
            Slot::Filled { item } => Some(item),
            _ => None,
        })
    }

    pub fn set_value(&mut self, id: usize, value: T) -> Result<(), ()> {
        let slot = self.values.get_mut(id).ok_or(())?;
        match slot {
            Slot::Empty { .. } => return Err(()),
            Slot::Filled { item } => item.value = value,
        }
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
        match self.values[id] {
            Slot::Empty { .. } => return Err(()),
            _ => {}
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
        match self.values[id] {
            Slot::Empty { .. } => return Err(()),
            _ => {}
        }

        let mut next = None;
        let mut previous = None;
        let mut current = self.first_free_index;

        while let Some(index) = current {
            if index > id {
                next = current;
                break;
            } else {
                previous = current;
            }

            match self.values.get(index) {
                Some(Slot::Empty {
                    next: next_index, ..
                }) => {
                    current = *next_index;
                }
                _ => return Err(()),
            }
        }

        let empty = Slot::new_empty(previous, next);
        self.values.push(empty);

        match previous.map_or(None, |i| self.values.get_mut(i)) {
            Some(Slot::Empty { next, .. }) => *next = Some(id),
            _ => {}
        }

        match next.map_or(None, |i| self.values.get_mut(i)) {
            Some(Slot::Empty { previous, .. }) => *previous = Some(id),
            _ => {}
        }

        let slot = self.values.swap_remove(id);

        match slot {
            Slot::Filled { item } => Ok(item),
            _ => Err(()),
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

                let node = self.get_node(node_idx).unwrap();
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

    pub fn to_vec(&self) -> Vec<(usize, &HashTreeNode<T>)> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|slot| match slot {
                (id, Slot::Filled { item }) => Some((id, item)),
                _ => None,
            })
            .collect::<_>()
    }

    pub fn to_vec_with_paths(&self) -> Vec<(Vec<usize>, &HashTreeNode<T>)> {
        self.to_vec()
            .into_iter()
            .map(|(id, node)| (self.get_node_path(id), node))
            .collect::<_>()
    }

    pub fn get_node_path(&self, id: usize) -> Vec<usize> {
        let mut path = vec![];
        let mut current = Some(id);

        while let Some(id) = current {
            path.push(id);
            let node = self.get_node(id).unwrap();
            current = node.parent;
        }

        path.reverse();
        path
    }
}

impl<T: Hash + Debug> HashTree<T> {
    #[rustfmt::skip]
    const INDENT_EMPTY   : &'static str = "    ";
    #[rustfmt::skip]
    const INDENT_STRAIGHT: &'static str = "│   ";
    #[rustfmt::skip]
    const INDENT_CHILD   : &'static str = "├───";
    #[rustfmt::skip]
    const INDENT_LAST    : &'static str = "└───";

    // const INDENT_EMPTY: &'static str = "    ";
    // const INDENT_STRAIGHT: &'static str = "┃   ";
    // const INDENT_CHILD: &'static str = "┣━━━";
    // const INDENT_LAST: &'static str = "┗━━━";

    fn debug_write_rec(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        id: usize,
        indent: String,
        prefix: String,
        last_flag: bool,
        top_level: bool,
    ) -> Result<(), std::fmt::Error> {
        let node = self.get_node(id).unwrap();
        writeln!(f, "{}{}{:?}", indent, prefix, node)?;
        if node.children.is_empty() {
            return Ok(());
        }

        let last = node.children.len() - 1;
        let mut result = Ok(());
        for (index, child) in node.children.iter().enumerate() {
            let prefix = if index == last {
                Self::INDENT_LAST
            } else {
                Self::INDENT_CHILD
            };

            let indent = if top_level {
                indent.clone()
            } else if last_flag {
                indent.clone() + Self::INDENT_EMPTY
            } else {
                indent.clone() + Self::INDENT_STRAIGHT
            };

            result =
                self.debug_write_rec(f, *child, indent, prefix.to_string(), index == last, false);

            if result.is_err() {
                break;
            }
        }

        result
    }
}

impl<T: Hash + Debug + PartialEq> HashTree<T> {
    pub fn difference(&self, other: &Self) -> Vec<Change> {
        let mut left = self.to_vec_with_paths();
        let mut right = other.to_vec_with_paths();
        let mut difference = vec![];

        for (l_path, l_node) in left.drain(..) {
            if let Some(matching) = right
                .iter()
                .position(|(r_path, r_node)| *r_path == l_path && **r_node == *l_node)
            {
                // no change
                right.remove(matching);
            } else if let Some(matching) = right.iter().position(|(_, r_node)| **r_node == *l_node)
            {
                let r_path = right[matching].0.clone();
                difference.push(Change::Update(
                    vec![],
                    Node {
                        kind: "file".to_string(),
                        value: Some(
                            r_path
                                .iter()
                                .map(|id| (*id).to_string())
                                .collect::<String>(),
                        ),
                        role: Some("moved".to_string()),
                    },
                ));
                right.remove(matching);
            } else if let Some(matching) = right.iter().position(|(r_path, _)| *r_path == l_path) {
                let r_path = right[matching].0.clone();
                difference.push(Change::Update(
                    vec![],
                    Node {
                        kind: "file".to_string(),
                        value: Some(
                            r_path
                                .iter()
                                .map(|id| (*id).to_string())
                                .collect::<String>(),
                        ),
                        role: Some("changed".to_string()),
                    },
                ));
                right.remove(matching);
            } else {
                difference.push(Change::Deletion(vec![]));
            }
        }

        // anytihng left over in right is an addition
        for (r_path, _) in right {
            difference.push(Change::Addition(
                vec![],
                Node {
                    kind: "file".to_string(),
                    value: Some(
                        r_path
                            .iter()
                            .map(|id| (*id).to_string())
                            .collect::<String>(),
                    ),
                    role: Some("new".to_string()),
                },
            ));
        }

        difference
    }
}

impl<T> Debug for HashTree<T>
where
    T: Hash + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root_id = match self.root_id {
            Some(id) => id,
            None => return writeln!(f, "Empty tree"),
        };

        self.debug_write_rec(f, root_id, "".to_string(), "".to_string(), false, true)
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::util::{BinaryFileRead, BinaryFileWrite};
    use super::HashTree;

    #[test]
    #[allow(unused)]
    fn it_works() {
        let mut tree = HashTree::default();
        tree.set_should_compute_hashes(false);

        let mut random = 17;

        println!("Inserting...");

        let root_id = tree.insert_root("root");

        for i in 0..1_000_000 {
            let parent = if i > 0 { random % i } else { 0 };
            random += 172742;

            let child_id = tree.insert(parent, "child").unwrap();
        }

        println!("Computing hashes...");

        tree.set_should_compute_hashes(true);

        // println!("{:?}", tree);
        println!("Saving...");

        let path = Path::new("test.tree");

        tree.write_to(path);
        println!("Loading...");

        let tree2 = HashTree::<String>::read_from(path);

        // println!("{:?}", tree2);

        println!("Done");
    }

    #[test]
    #[allow(unused)]
    fn test_difference() {
        let mut tree = HashTree::default();
        tree.set_should_compute_hashes(false);

        let mut random = 17;

        let root_id = tree.insert_root("root");

        for i in 0..1_000 {
            let parent = if i > 0 { random % i } else { 0 };
            random += 172742;

            let child_id = tree.insert(parent, "child").unwrap();
        }
        tree.refresh_hashes();

        let mut tree2 = HashTree::default();
        tree2.set_should_compute_hashes(false);

        let root_id = tree2.insert_root("root");
        let mut random = 17;

        for i in 0..1_000 {
            let parent = if i > 0 { random % i } else { 0 };
            random += 172742;

            let child_id = tree2.insert(parent, "child").unwrap();
        }
        tree2.refresh_hashes();


        let difference = tree.difference(&tree2);

        for d in difference.iter() {
            println!("{:?}", d);
        }

        assert_eq!(difference.len(), 0);

        let child_id = tree2.insert(0, "child").unwrap();

        let difference = tree.difference(&tree2);

        for d in difference.iter() {
            println!("{:?}", d);
        }

        assert_eq!(difference.len(), 1);
    }
}
