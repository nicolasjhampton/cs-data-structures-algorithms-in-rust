mod nodes;
pub mod linked;

use nodes::linked_list::Node;
use std::{ cell::RefCell, rc::{Rc, Weak}};

pub trait List: Iterator {

    fn curr(&self) -> Option<Rc<RefCell<Node>>>;

    #[allow(dead_code)]
    fn unshift(&mut self, new_head: &str) {
        let next = match self.curr() {
            Some(ref curr) => Some(Rc::clone(curr)),
            None => None
        };
    }

    fn shift(&mut self) {
        let next = match self.curr() {
            Some(ref node) => node.borrow().next(),
            None => None
        };
    }
}