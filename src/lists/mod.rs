mod nodes;
pub mod linked;

use nodes::linked_list::Node;
use std::{ cell::RefCell, rc::{Rc}};


pub trait Stack: Iterator {

    fn current(&self) -> Option<Rc<RefCell<Node>>>;

    fn set_curr(&mut self, node: Option<Rc<RefCell<Node>>>);

    fn unshift(&mut self, data: &str) {
        if let Some(current) = self.current() {
            self.set_curr(Some(Node::new(data, Some(current))));
        };
    }

    fn shift(&mut self) -> Option<String> {
        let current_value = match self.current() {
            Some(ref node) => Some(node.borrow().value()),
            None => None
        };
        let next = match self.current() {
            Some(ref node) => node.borrow().next(),
            None => None
        };
        match next {
            Some(ref node) => self.set_curr(Some(Rc::clone(node))),
            None => self.set_curr(None)
        };
        current_value
    }
}