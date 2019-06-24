use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct LinkedListNode {
    value: String,
    next: Option<Rc<RefCell<LinkedListNode>>>
}

impl LinkedListNode {
    pub fn new(value: &str) -> LinkedListNode {
        LinkedListNode {
            value: String::from(value),
            next: None
        }
    }

    pub fn next(&self) -> Option<Rc<RefCell<LinkedListNode>>> {
        match &self.next {
            Some(node) => Some(Rc::clone(&node)),
            None => None
        }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

struct LinkedList {
    head: Weak<RefCell<LinkedListNode>>,
    curr: Option<Rc<RefCell<LinkedListNode>>>
}

// impl LinkedList {
//     fn new() {

//     }

//     fn add() {

//     }
// }

impl Iterator for LinkedList {
    type Item = Rc<RefCell<LinkedListNode>>;

    fn next(&mut self) -> Option<Rc<RefCell<LinkedListNode>>>  {
        self.curr = match &self.curr {
            Some(curr) => (*curr.borrow()).next(),
            None => None
        };
        match &self.curr {
            Some(curr) => Some(Rc::clone(curr)),
            None => None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = LinkedListNode {
            value: String::from("tonia"),
            next: None
        };

        assert_eq!("tonia", node.value);
    }

    #[test]
    fn new_creates_node() {
        let node = LinkedListNode::new("tonia");
        assert_eq!("tonia", node.value);
    }

    #[test]
    fn next_gives_ref() {
        let mut node = Rc::new(RefCell::new(LinkedListNode {
            value: String::from("tonia"),
            next: Some(
                Rc::new(
                    RefCell::new(
                        LinkedListNode {
                            value: String::from("nic"),
                            next: Some(
                                Rc::new(
                                    RefCell::new(
                                        LinkedListNode {
                                            value: String::from("bill"),
                                            next: None,
                                        }
                                    )
                                )
                            )
                        }
                    )
                )
            )
        }));

        let names = ["tonia", "nic",  "bill"];

        for name in names.iter() {
            assert_eq!(*Rc::clone(&node).borrow().value(), name.to_string());
            match Rc::clone(&node).borrow().next() {
                Some(new_node) => node = new_node,
                None => break
            }
        }
    }

    #[test]
    fn linked_list_is_iterator() {
        let node = Rc::new(RefCell::new(LinkedListNode {
            value: String::from("first"),
            next: None
        }));
        let mut list = LinkedList {
            head: Rc::downgrade(&node),
            curr: Some(node)
        };
        for node in list {
            assert_eq!(*node.borrow().value(), "first".to_string());
        }
    }
}
