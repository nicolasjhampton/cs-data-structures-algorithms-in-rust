use std::{ cell::RefCell, rc::{ Rc, Weak } };

use super::{ RefExt, DoubleRefExt, CreateRefExt };

pub type DoubleNodeRef = Rc<RefCell<DoubleNode>>;
pub type WeakDoubleNodeRef = Weak<RefCell<DoubleNode>>;

impl CreateRefExt for DoubleNodeRef {
    type Reference = DoubleNodeRef;
    type Node = DoubleNode;

    fn from_node(node: DoubleNode) -> DoubleNodeRef {
        Rc::new(RefCell::new(node))
    }
}

impl RefExt for DoubleNodeRef {
    type Reference = DoubleNodeRef;

    fn next(&self) -> Option<DoubleNodeRef> {
        self.borrow().next()
    }

    fn set_next(&mut self, next: Option<DoubleNodeRef>) {
        self.borrow_mut().next = next;
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

    fn set_prev(&mut self, prev: Option<Self::WeakReference>) {
        self.borrow_mut().prev = prev;
    }

    fn weak(&self) -> WeakDoubleNodeRef {
        Rc::downgrade(self)
    }
}

/*
 *  DoubleNode
 * 
 *  Lowest level of the Double Linked List
 *  Returns only References of itself to hide
 *  internals.
 * 
 */
#[derive(Debug)]
pub struct DoubleNode {
    value: String,
    next: Option<DoubleNodeRef>,
    prev: Option<WeakDoubleNodeRef>,
}

impl DoubleNode {
    pub fn new(value: &str, next: Option<DoubleNodeRef>, prev: Option<WeakDoubleNodeRef>) -> DoubleNodeRef {
        let mut node_ref = DoubleNodeRef::from_node(
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

        node_ref.set_next(next);
        node_ref.set_prev(prev);

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
            assert_eq!(node.value(), name.to_string());
            if let Some(new_node) = node.next() {
                node = new_node;
            } else {
                break;
            }
        }
    }

    #[test]
    fn prev_gives_ref() {
        let first_node = DoubleNode::new("tonia", None, None);
        let second_node = DoubleNode::new("nic", None, Some(first_node.weak()));
        let mut last_node = DoubleNode::new("bill", None, Some(second_node.weak()));

        let reverse_names = ["bill", "nic", "tonia"];

        for name in reverse_names.iter() {
            assert_eq!(last_node.value(), name.to_string());
            if let Some(new_node) = last_node.prev() {
                last_node = new_node.upgrade().unwrap();
            } else {
                break;
            }
        }
    }
}
