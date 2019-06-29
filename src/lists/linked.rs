use std::{ cell::RefCell, rc::{Rc, Weak}};

use super::nodes::linked_list::Node;

use super::List;

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
}

impl List for LinkedList {
    fn curr(&self) -> Option<Rc<RefCell<Node>>> {
        match self.curr {
            Some(ref curr) => Some(Rc::clone(curr)),
            None => None
        }
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
        list.unshift("second");
        list.unshift("third");
        let sequence = ["third", "second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(*node.borrow().value(), sequence[index].to_string());
        }
    }

    #[test]
    fn remove_erases_element_from_list() {
        let mut list = LinkedList::new("first");
        list.unshift("second");
        list.unshift("third");
        list.shift();
        let sequence = ["second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(*node.borrow().value(), sequence[index].to_string());
        }
    }
}
