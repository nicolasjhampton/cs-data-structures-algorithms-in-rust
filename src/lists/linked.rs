use std::{ cell::RefCell, rc::{Rc, Weak}};

use super::nodes::linked_list::Node;

pub struct LinkedList {
    pub head: RefCell<Weak<RefCell<Node>>>,
    pub curr: Option<Rc<RefCell<Node>>>
}

impl LinkedList {
    #[allow(dead_code)]
    fn new(start: &str) -> LinkedList {
        let tail = Rc::new(RefCell::new(Node::new(start, None)));
        LinkedList {
            head: RefCell::new(Rc::downgrade(&tail)),
            curr: Some(tail)
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, new_head: &str) {
        let next = match self.curr {
            Some(ref curr) => Some(Rc::clone(curr)),
            None => None
        };
        let head_node = Node::new(new_head, next);
        let head_ref = Rc::new(RefCell::new(head_node));
        *self.head.borrow_mut() = Rc::downgrade(&head_ref);
    }

    fn remove(&mut self) {
        let next = match self.curr {
            Some(ref node) => node.borrow().next(),
            None => None
        };
        if let Some(node) = next {
            *self.head.borrow_mut() = Rc::downgrade(&node);
            self.curr = Some(node);
        };
    }
}

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
        let node = Rc::new(RefCell::new(Node::new("first", None)));
        let list = LinkedList {
            head: RefCell::new(Rc::downgrade(&node)),
            curr: Some(node)
        };
        for node in list {
            assert_eq!(*node.borrow().value(), "first".to_string());
        }
    }

    #[test]
    fn new_creates_linked_list() {
        let list = LinkedList::new("first");
        for node in list {
            assert_eq!(*node.borrow().value(), "first".to_string());
        }
    }

    #[test]
    fn add_adds_element_to_list() {
        let mut list = LinkedList::new("first");
        list.add("second");
        list.add("third");
        let sequence = ["third", "second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(*node.borrow().value(), sequence[index].to_string());
        }
    }

    #[test]
    fn remove_erases_element_from_list() {
        let mut list = LinkedList::new("first");
        list.add("second");
        list.add("third");
        list.remove();
        let sequence = ["second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(*node.borrow().value(), sequence[index].to_string());
        }
    }
}
