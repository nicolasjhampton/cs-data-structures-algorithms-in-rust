use std::{ cell::RefCell, rc::{ Rc, Weak } };

pub struct Node {
    pub data: i32,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(data: i32) -> Node {
        Node {
            data,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn walk_l_d_r(&self) {
        if let Some(child) = &self.left {
            child.borrow().walk_l_d_r();
        };
        println!("{}", self.data);
        if let Some(child) = &self.right {
            child.borrow().walk_l_d_r();
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node {
            data: 0,
            parent: None,
            left: None,
            right: None,
        };

        assert_eq!(0, node.data);
    }

    #[test]
    fn new_creates_node() {
        let node = Node::new(1);
        assert_eq!(1, node.data);
    }
}