mod nodes;
pub mod linked;
pub mod doubly_linked;

use nodes::linked_list::Node;
use std::{ cell::RefCell, rc::{Rc}};


pub trait Stack: Iterator {
    type Reference;
    fn head(&self) -> Option<Self::Reference>;
    fn set_head(&mut self, node: Option<Self::Reference>);
    fn shift(&mut self) -> Option<String>;
    fn unshift(&mut self, data: &str);
}