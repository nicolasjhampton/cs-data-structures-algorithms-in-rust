use std::{ cell::RefCell, rc::{ Rc, Weak } };

use super::{ RefExt, DoubleRefExt, CreateDoubleRefExt };

pub type DoubleNodeRef = Rc<RefCell<DoubleNode>>;
pub type WeakDoubleNodeRef = Weak<RefCell<DoubleNode>>;

impl CreateDoubleRefExt for DoubleNodeRef {
    type WeakReference = WeakDoubleNodeRef;
    type Reference = DoubleNodeRef;
    type Node = DoubleNode;

    fn new_ref(value: &str, next: Option<DoubleNodeRef>, prev: Option<WeakDoubleNodeRef>) -> DoubleNodeRef {
        DoubleNode::new(value, next, prev)
    }

    fn from_node(node: DoubleNode) -> DoubleNodeRef {
        Rc::new(RefCell::new(node))
    }
}

impl RefExt for DoubleNodeRef {
    type Reference = DoubleNodeRef;

    fn next(&self) -> Option<DoubleNodeRef> {
        self.borrow().next()
    }

    fn value(&self) -> String {
        self.borrow().value()
    }

    fn refer(&self) -> DoubleNodeRef {
        Rc::clone(self)
    }
}

impl DoubleRefExt for DoubleNodeRef {
    type WeakReference = WeakDoubleNodeRef;

    fn prev(&self) -> Option<WeakDoubleNodeRef> {
        self.borrow().prev()
    }

    fn weak(&self) -> WeakDoubleNodeRef {
        Rc::downgrade(self)
    }
}

#[derive(Debug)]
pub struct DoubleNode {
    pub value: String,
    pub next: Option<DoubleNodeRef>,
    pub prev: Option<WeakDoubleNodeRef>,
}

impl DoubleNode {
    pub fn new(value: &str, next: Option<DoubleNodeRef>, prev: Option<WeakDoubleNodeRef>) -> DoubleNodeRef {
        let node_ref = DoubleNodeRef::from_node(
            DoubleNode {
                value: String::from(value),
                next: None,
                prev: None
            }
        );
        
        if let Some(next_node) = &next {
            next_node.borrow_mut().prev = Some(node_ref.weak());
        };

        if let Some(prev_node) = &prev {
            if let Some(prev_ref) = prev_node.upgrade() {
                prev_ref.borrow_mut().next = Some(node_ref.refer());
            };
        };

        node_ref.borrow_mut().next = next;
        node_ref.borrow_mut().prev = prev;
        node_ref
    }

    pub fn next(&self) -> Option<DoubleNodeRef> {
        match &self.next {
            Some(node) => Some(node.refer()),
            None => None
        }
    }

    pub fn prev(&self) -> Option<WeakDoubleNodeRef> {
        match &self.prev {
            Some(node) => Some(node.clone()),
            None => None
        }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = DoubleNode {
            value: String::from("tonia"),
            next: None,
            prev: None
        };

        assert_eq!("tonia", node.value);
    }

    #[test]
    fn new_creates_node() {
        let node = DoubleNode::new("tonia", None, None);
        assert_eq!("tonia", node.value());
    }

    #[test]
    fn value_returns_string() {
        let node = DoubleNode::new("tonia", None, None);
        assert_eq!("tonia".to_string(), node.value());
    }

    #[test]
    fn next_gives_ref() {
        let mut node = 
        DoubleNode::new("tonia", Some( 
            DoubleNode::new("nic", Some( 
                DoubleNode::new("bill", None, None) 
            ), None) 
        ), None);

        let names = ["tonia", "nic",  "bill"];

        for name in names.iter() {
            assert_eq!(*Rc::clone(&node).value(), name.to_string());
            match Rc::clone(&node).next() {
                Some(new_node) => node = new_node,
                None => break
            }
        }
    }
}
