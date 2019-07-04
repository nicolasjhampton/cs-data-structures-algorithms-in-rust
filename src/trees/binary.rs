use std::{ thread, rc::{ Rc }};

use std::sync::Arc;

use std::sync::mpsc::{Sender, Receiver, channel};

use super::nodes::{ RefExt, binary_tree::{ Node, NodeRef }};

#[derive(Debug)]
pub struct BinaryTree {
    // pub root: Option<NodeRef>,
    // curr: Option<NodeRef>,
    iter: Receiver<String>
}

impl BinaryTree {
    #[allow(dead_code)]
    fn new(start: &str) -> BinaryTree {
        let (tx, rx) = channel();
        let string_start = start.to_string();
        thread::spawn(|| {
            let root = Node {
                data: string_start,
                left: None,
                right: None
            };
            root.depth_walk(tx);
        });
        BinaryTree {
            iter: rx
        }
    }

    // fn set_curr(&mut self, node: Option<NodeRef>) {
    //     self.curr = node;
    // }

    // fn curr(&self) -> Option<NodeRef> {
    //     match self.curr {
    //         Some(ref curr) => Some(Rc::clone(curr)),
    //         None => None
    //     }
    // }
}

impl Iterator for BinaryTree {
    type Item = String;

    fn next(&mut self) -> Option<String>  {
        match self.iter.recv() {
            Ok(value) => Some(value),
            _ => None
        }
    }
}

// impl  LinkedList {

//     type Reference = NodeRef;

//     fn set_curr(&mut self, node: Option<NodeRef>) {
//         self.head = node;
//     }

//     fn head(&self) -> Option<NodeRef> {
//         match self.head {
//             Some(ref head) => Some(Rc::clone(head)),
//             None => None
//         }
//     }

//     fn unshift(&mut self, data: &str) {
//         if let Some(head) = self.head() {
//             self.set_head(Some(Node::new(data, Some(head))));
//         };
//     }

//     fn shift(&mut self) -> Option<String> {
//         let current_value = match self.head() {
//             Some(ref node) => Some(node.borrow().value()),
//             None => None
//         };
//         let next = match self.head() {
//             Some(ref node) => node.borrow().next(),
//             None => None
//         };
//         match next {
//             Some(ref node) => self.set_head(Some(Rc::clone(node))),
//             None => self.set_head(None)
//         };
//         current_value
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_tree_is_iterator() {
        let list = BinaryTree::new("first");
        // let list = BinaryTree {
        //     root: Some(Rc::clone(&node)),
        //     curr: Some(Rc::clone(&node)),
        //     switch: true
        // };
        for node in list {
            assert_eq!(node, "first".to_string());
        }
    }

    // #[test]
    // fn binary_tree_walks_l_d_r_walks_correctly() {
    //     let list = BinaryTree::new("first");
    //     list.
    //     for node in list {
    //         assert_eq!(node, "first".to_string());
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn new_creates_linked_list() {
    //     let list = BinaryTree::new("first");
    //     for node in list {
    //         assert_eq!(node, "first".to_string());
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn unshift_adds_element_to_list() {
    //     let mut list = BinaryTree::new("first");
    //     list.unshift("second");
    //     list.unshift("third");
    //     let sequence = ["third", "second", "first"];
    //     for (index, node) in list.enumerate() {
    //         assert_eq!(node, sequence[index].to_string());
    //     }
    // }

    // #[test]
    // #[ignore]
    // fn remove_removes_element_from_list() {
    //     let mut list = BinaryTree::new("first");
    //     list.unshift("second");
    //     list.unshift("third");
    //     let third = list.shift();
    //     assert_eq!(third.unwrap(), "third".to_string());
    //     let sequence = ["second", "first"];
    //     for (index, node) in list.enumerate() {
    //         assert_eq!(node, sequence[index].to_string());
    //     }
    // }
}
