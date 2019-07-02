use std::{ cell::RefCell, rc::{ Rc } };

#[derive(Debug)]
pub struct Node {
    pub data: String,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(data: &str) -> Node {
        Node {
            data: String::from(data),
            left: None,
            right: None,
        }
    }

    #[allow(dead_code)]
    fn walk_l_d_r(&self, coll: &mut Vec<String>) {
        match &self.left {
            Some(child) => child.borrow().walk_l_d_r(coll),
            None => ()
        };
        coll.push(self.data.clone());
        match &self.right {
            Some(child) => child.borrow().walk_l_d_r(coll),
            None => ()
        };
    }

    #[allow(dead_code)]
    fn insert(&mut self, node: Rc<RefCell<Node>>) {
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
        assert_eq!(1.to_string(), node.data);
    }

    #[test]
    fn walk_l_d_r_walks_tree() {
        let node3 = Rc::new(RefCell::new(Node::new("3")));
        let node0 = Rc::new(RefCell::new(Node::new("0")));
        let node1 = Rc::new(RefCell::new(Node::new("1")));
        let node2 = Rc::new(RefCell::new(Node::new("2")));
        let node4 = Rc::new(RefCell::new(Node::new("4")));
        let node5 = Rc::new(RefCell::new(Node::new("5")));
        let node6 = Rc::new(RefCell::new(Node::new("6")));

        node1.borrow_mut().left = Some(Rc::clone(&node0));
        node1.borrow_mut().right = Some(Rc::clone(&node2));
        node3.borrow_mut().left = Some(Rc::clone(&node1));
        node5.borrow_mut().left = Some(Rc::clone(&node4));
        node5.borrow_mut().right = Some(Rc::clone(&node6));
        node3.borrow_mut().right = Some(Rc::clone(&node5));

        let mut arr : Vec<String> = Vec::new();
        node3.borrow().walk_l_d_r(&mut arr);
        for (i, j) in arr.iter().enumerate() {
            assert_eq!(i.to_string(), *j);
        }
    }

    #[test]
    fn insert_keeps_binary_sort() {
        let nodes : Vec<usize> = vec![2, 5, 0, 1, 4, 6];
        let mut node3 = Node::new("3");
        for i in nodes.iter() {
            node3.insert(Rc::new(RefCell::new(Node::new(&i.to_string()))));
        }
        let mut arr : Vec<String> = Vec::new();
        node3.walk_l_d_r(&mut arr);
        for (i, j) in arr.iter().enumerate() {
            assert_eq!(i.to_string(), *j);
        }
    }
}