use std::cell::RefCell;

use super::nodes::{ BinaryNode, node::Node, node_ref::NodeRef };

use crate::lists::{ Stack, doubly_linked::DoublyLinkedList as QueueList };

#[derive(Debug)]
pub struct BinaryTree {
    pub root: NodeRef,
    iter: Option<RefCell<QueueList>>
}

impl BinaryTree {
    #[allow(dead_code)]
    pub fn new(start: &str) -> BinaryTree {
        let node = Node::new(start);
        BinaryTree {
            root: node,
            iter: None
        }
    }

    pub fn add(&mut self, new: &str) {
        self.root.borrow_mut().insert(Node::new(new));
    }

    fn to_iter(&mut self) {
        let mut coll = QueueList::new();
        self.root.borrow().depth_walk(&mut coll);
        self.iter = Some(RefCell::new(coll));
    }

}

impl Iterator for BinaryTree {
    type Item = String;

    fn next(&mut self) -> Option<String>  {
        match &self.iter {
            Some(iter) => iter.borrow_mut().shift(),
            None => {
                self.to_iter();
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tree_is_iterator() {
        let mut list = BinaryTree::new("first");
        list.add("a");
        list.add("c");
        list.add("b");
        list.add("zz");
        let answers = ["a", "b", "c", "first", "zz"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, answers[index].to_string());
        }
    }
}
