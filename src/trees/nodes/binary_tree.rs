use std::{ thread, sync::mpsc::{ channel, Sender }, rc::Rc, cell::RefCell };

use crate::lists::{ Queue, doubly_linked::DoublyLinkedList as QueueList };

use super::{ CreateRefExt, RefExt };

pub type NodeRef = Rc<RefCell<Node>>;

impl CreateRefExt for NodeRef {
    type Node = Node;
    type Reference = NodeRef;

    fn from_node(node: Node) -> NodeRef {
        Rc::new(RefCell::new(node))
    }
}

impl RefExt for NodeRef {
    type Reference = NodeRef;

    fn value(&self) -> String {
        self.borrow().value()
    }

    fn refer(&self) -> NodeRef {
        Rc::clone(self)
    }

    fn left(&self) -> Option<NodeRef> {
        self.borrow().left()
    }

    fn set_left(&mut self, left: Option<NodeRef>) {
        self.borrow_mut().left = left;
    }

    fn right(&self) -> Option<NodeRef> {
        self.borrow().right()
    }

    fn set_right(&mut self, right: Option<NodeRef>) {
        self.borrow_mut().right = right;
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub data: String,
    pub left: Option<NodeRef>,
    pub right: Option<NodeRef>
}

impl Node {
    pub fn new(data: &str) -> NodeRef {
        NodeRef::from_node(Node {
            data: String::from(data),
            left: None,
            right: None,
        })
    }

    fn value(&self) -> String {
        self.data.clone()
    }

    fn left(&self) -> Option<NodeRef> {
        match &self.left {
            Some(ref child) => Some(child.refer()),
            None => None
        }
    }

    fn right(&self) -> Option<NodeRef> {
        match &self.right {
            Some(ref child) => Some(child.refer()),
            None => None
        }
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, node: NodeRef) {
        if self.data >= node.borrow().data {
            if let Some(left) = &self.left {
                left.borrow_mut().insert(Rc::clone(&node));
            } else {
                self.left = Some(Rc::clone(&node));
            }
        } else {
            if let Some(right) = &self.right {
                right.borrow_mut().insert(Rc::clone(&node));
            } else {
                self.right = Some(Rc::clone(&node));
            }
        }
    }

    #[allow(dead_code)]
    pub fn depth_walk(&self, coll: &mut QueueList) {
        match &self.left {
            Some(child) => child.borrow().depth_walk(coll),
            None => ()
        };
        coll.push(&self.value());
        match &self.right {
            Some(child) => child.borrow().depth_walk(coll),
            None => ()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node {
            data: "0".to_string(),
            left: None,
            right: None,
        };

        assert_eq!("0".to_string(), node.data);
    }

    #[test]
    fn new_creates_node() {
        let node = Node::new("1");
        assert_eq!(1.to_string(), node.value());
    }

    #[test]
    fn insert_keeps_binary_sort() {
        let aought_to_be_nodes : Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6];
        let nodes : Vec<usize> = vec![2, 5, 0, 1, 4, 6];
        let node = Node::new("3");
        for i in nodes.iter() {
            node.borrow_mut().insert(Node::new(&i.to_string()));
        }
        let mut coll = QueueList::new();
        node.clone().borrow().depth_walk(&mut coll);
        for (index, value) in coll.enumerate() {
            println!("{:#?}", value);
            assert_eq!(index.to_string(), *value);
        }
    }
}