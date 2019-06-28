use std::{ cell::RefCell, rc::{ Rc, Weak } };

pub struct Node {
    pub data: usize,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(data: usize) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    fn walk_l_d_r(&self, coll: &mut Vec<usize>) {
        match &self.left {
            Some(child) => child.borrow().walk_l_d_r(coll),
            None => ()
        };
        coll.push(self.data);
        match &self.right {
            Some(child) => child.borrow().walk_l_d_r(coll),
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
            data: 0,
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

    #[test]
    fn walk_l_d_r_walks_tree() {
        let node3 = Rc::new(RefCell::new(Node::new(3)));
        let node0 = Rc::new(RefCell::new(Node::new(0)));
        let node1 = Rc::new(RefCell::new(Node::new(1)));
        let node2 = Rc::new(RefCell::new(Node::new(2)));
        let node4 = Rc::new(RefCell::new(Node::new(4)));
        let node5 = Rc::new(RefCell::new(Node::new(5)));
        let node6 = Rc::new(RefCell::new(Node::new(6)));

        node1.borrow_mut().left = Some(Rc::clone(&node0));
        node1.borrow_mut().right = Some(Rc::clone(&node2));
        node3.borrow_mut().left = Some(Rc::clone(&node1));
        node5.borrow_mut().left = Some(Rc::clone(&node4));
        node5.borrow_mut().right = Some(Rc::clone(&node6));
        node3.borrow_mut().right = Some(Rc::clone(&node5));
        let mut arr : Vec<usize> = Vec::new();
        node3.borrow().walk_l_d_r(&mut arr);
        for (i, j) in arr.iter().enumerate() {
            assert_eq!(i, *j);
        }
    }
}