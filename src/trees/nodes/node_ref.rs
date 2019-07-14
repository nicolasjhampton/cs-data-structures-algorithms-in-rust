use std::{ rc::Rc, cell::RefCell };

use super::{ TreeNode, RefExt, node::Node };

pub type NodeRef = Rc<RefCell<Node>>;

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
        self.borrow_mut().set_left(left);
    }

    fn right(&self) -> Option<NodeRef> {
        self.borrow().right()
    }

    fn set_right(&mut self, right: Option<NodeRef>) {
        self.borrow_mut().set_right(right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        // let node = Node {
        //     data: "0".to_string(),
        //     left: None,
        //     right: None,
        // };

        // assert_eq!("0".to_string(), node.value());
    }
}