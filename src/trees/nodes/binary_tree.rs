use std::{ thread, sync::mpsc::{ channel, Sender }, rc::Rc, cell::RefCell };

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
    fn insert(&mut self, node: NodeRef) {
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
    pub fn depth_walk(&self, pipe: Sender<String>) {
        match &self.left {
            Some(child) => child.borrow().depth_walk(Sender::clone(&pipe)),
            None => ()
        };
        pipe.send(self.value()).unwrap();
        match &self.right {
            Some(child) => child.borrow().depth_walk(Sender::clone(&pipe)),
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

    // #[test]
    // fn walk_l_d_r_walks_tree() {
    //     let node3 = Rc::new(RefCell::new(Node::new("3")));
    //     let node0 = Rc::new(RefCell::new(Node::new("0")));
    //     let node1 = Rc::new(RefCell::new(Node::new("1")));
    //     let node2 = Rc::new(RefCell::new(Node::new("2")));
    //     let node4 = Rc::new(RefCell::new(Node::new("4")));
    //     let node5 = Rc::new(RefCell::new(Node::new("5")));
    //     let node6 = Rc::new(RefCell::new(Node::new("6")));

    //     node1.borrow_mut().left = Some(Rc::clone(&node0));
    //     node1.borrow_mut().right = Some(Rc::clone(&node2));
    //     node3.borrow_mut().left = Some(Rc::clone(&node1));
    //     node5.borrow_mut().left = Some(Rc::clone(&node4));
    //     node5.borrow_mut().right = Some(Rc::clone(&node6));
    //     node3.borrow_mut().right = Some(Rc::clone(&node5));

    //     let mut arr : Vec<String> = Vec::new();
    //     node3.borrow().walk_l_d_r(&mut arr);
    //     for (i, j) in arr.iter().enumerate() {
    //         assert_eq!(i.to_string(), *j);
    //     }
    // }

    #[test]
    fn insert_keeps_binary_sort() {
        let mut i = 0;
        let (tx, rx) = channel();
        let aought_to_be_nodes : Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6];
        thread::spawn(move || {
            let nodes : Vec<usize> = vec![2, 5, 0, 1, 4, 6];
            let node = Node::new("3");
            for i in nodes.iter() {
                node.borrow_mut().insert(Node::new(&i.to_string()));
            }
            node.clone().borrow().depth_walk(tx);
        });
        for value in rx {
            println!("{:#?}", value);
            assert_eq!(aought_to_be_nodes[i].to_string(), value);
            i += 1;
        }
    }
}