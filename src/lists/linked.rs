use std::{ cell::RefCell, rc::{Rc, Weak}};

use super::nodes::linked_list::Node;

pub struct LinkedList {
    pub head: Weak<RefCell<Node>>,
    pub curr: Option<Rc<RefCell<Node>>>
}

// impl LinkedList {
//     fn new() {

//     }

//     fn add() {

//     }
// }

impl Iterator for LinkedList {
    type Item = Rc<RefCell<Node>>;

    fn next(&mut self) -> Option<Rc<RefCell<Node>>>  {
        self.curr = match self.curr {
            Some(ref curr) => curr.borrow().next(),
            None => None
        };
        match self.curr {
            Some(ref curr) => Some(Rc::clone(&curr)),
            None => None
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_is_iterator() {
        let node = Rc::new(RefCell::new(Node {
            value: String::from("first"),
            next: None
        }));
        let list = LinkedList {
            head: Rc::downgrade(&node),
            curr: Some(node)
        };
        for node in list {
            assert_eq!(*node.borrow().value(), "first".to_string());
        }
    }
}
