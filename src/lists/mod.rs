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

pub trait Queue: Stack {
    type WeakReference;
    fn tail(&self) -> Option<Self::Reference>;
    fn set_tail(&mut self, node: Self::WeakReference);
    fn pop(&mut self) -> Option<String>;
    fn push(&mut self, data: &str);
}