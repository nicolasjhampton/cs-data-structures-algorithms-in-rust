pub mod linked_list;
pub mod doubly_linked_list;

use std::{ cell::RefCell, rc::Rc };

use linked_list::Node;

pub trait RefExt {
    type Reference;

    fn next(&self) -> Option<Self::Reference>;
    fn value(&self) -> String;
    fn refer(&self) -> Self::Reference;
}

pub trait DoubleRefExt: RefExt {
    type WeakReference;

    fn prev(&self) -> Option<Self::WeakReference>;
    fn weak(&self) -> Self::WeakReference;
}

pub trait CreateRefExt {
    type Reference;

    fn new_ref(value: &str, next: Option<Self::Reference>) -> Self::Reference;
}

pub trait CreateDoubleRefExt {
    type WeakReference;
    type Reference;
    type Node;

    fn new_ref(value: &str, next: Option<Self::Reference>, prev: Option<Self::WeakReference>) -> Self::Reference;
    fn from_node(node: Self::Node) -> Self::Reference;
}



pub type NodeRef = Rc<RefCell<Node>>;

impl CreateRefExt for NodeRef {
    type Reference = NodeRef;

    fn new_ref(value: &str, next: Option<NodeRef>) -> NodeRef {
        Rc::new(RefCell::new(Node::new(value, next)))
    }
}

impl RefExt for NodeRef {
    type Reference = NodeRef;

    fn next(&self) -> Option<NodeRef> {
        self.borrow().next()
    }

    fn value(&self) -> String {
        self.borrow().value()
    }

    fn refer(&self) -> NodeRef {
        Rc::clone(self)
    }
}



