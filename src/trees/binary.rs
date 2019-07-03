use std::{ rc::{ Rc }};

use super::nodes::binary_tree::{ Node, NodeRef };

#[derive(Debug)]
pub struct BinaryTree {
    pub root: Option<NodeRef>,
    curr: Option<NodeRef>
}

impl BinaryTree {
    #[allow(dead_code)]
    fn new(start: &str) -> BinaryTree {
        let root = Node::new(start);
        BinaryTree {
            root: Some(root)
        }
    }

    fn set_curr(&mut self, node: Option<NodeRef>) {
        self.curr = node;
    }

    fn curr(&self) -> Option<NodeRef> {
        match self.curr {
            Some(ref curr) => Some(Rc::clone(curr)),
            None => None
        }
    }
}

// impl Iterator for LinkedList {
//     type Item = String;

//     fn next(&mut self) -> Option<String>  {
//         self.shift()
//     }
// }

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
    fn linked_list_is_iterator() {
        let node = Node::new("first", None);
        let list = LinkedList {
            head: Some(node)
        };
        for node in list {
            assert_eq!(node, "first".to_string());
        }
    }

    #[test]
    fn new_creates_linked_list() {
        let list = LinkedList::new("first");
        for node in list {
            assert_eq!(node, "first".to_string());
        }
    }

    #[test]
    fn unshift_adds_element_to_list() {
        let mut list = LinkedList::new("first");
        list.unshift("second");
        list.unshift("third");
        let sequence = ["third", "second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }

    #[test]
    fn remove_removes_element_from_list() {
        let mut list = LinkedList::new("first");
        list.unshift("second");
        list.unshift("third");
        let third = list.shift();
        assert_eq!(third.unwrap(), "third".to_string());
        let sequence = ["second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }
}
