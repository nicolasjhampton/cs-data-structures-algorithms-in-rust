use std::{ cell::RefCell, rc::{ Rc }};

use std::sync::mpsc::{Sender, Receiver, channel};

use super::nodes::{ RefExt, binary_tree::{ Node, NodeRef }};

#[derive(Debug)]
pub struct BinaryTree {
    pub root: NodeRef,
    iter: Option<RefCell<Vec<String>>>
}

impl BinaryTree {
    #[allow(dead_code)]
    fn new(start: &str) -> BinaryTree {
        let node = Node::new(start);
        BinaryTree {
            root: node,
            iter: None
        }
    }

    fn add(&mut self, new: &str) {
        self.root.borrow_mut().insert(Node::new(new));
    }

    fn to_iter(&mut self) {
        let mut coll = Vec::new();
        self.root.borrow().depth_walk(&mut coll);
        self.iter = Some(RefCell::new(coll));
    }

}

impl Iterator for BinaryTree {
    type Item = String;

    fn next(&mut self) -> Option<String>  {
        match self.iter {
            Some(ref iter) => iter.borrow_mut().pop(),
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
        let answers = ["zz", "first", "c", "b", "a"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, answers[index].to_string());
        }
    }
}
