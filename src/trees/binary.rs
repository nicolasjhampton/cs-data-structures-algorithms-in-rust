use std::{ cell::RefCell, rc::{ Rc, Weak }};

use super::nodes::binary_tree::Node;

struct BinaryTree {
    root: Rc<RefCell<Node>>
}

impl BinaryTree {
    fn new(initial_data: i32) -> BinaryTree {
        BinaryTree {
            root: Rc::new(RefCell::new(Node::new(initial_data)))
        }
    }

    // fn walk_l_d_r(&self) {
    //     self.root.borrow().walk_l_d_r();
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_tree() {
        let tree = BinaryTree {
            root: Rc::new(RefCell::new(Node::new(0)))
        };

        assert_eq!(tree.root.borrow().data, 0);
    }

    #[test]
    fn new_creates_tree() {
        let tree = BinaryTree::new(1);
        assert_eq!(tree.root.borrow().data, 1);
    }
}