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
    fn shift(&mut self, new_head: &str) {
        let next = match self.curr {
            Some(ref curr) => Some(Rc::clone(curr)),
            None => None
        };
        let head_node = Node::new(new_head, next);
        let head_ref = Rc::new(RefCell::new(head_node));
        *self.head.borrow_mut() = Rc::downgrade(&head_ref);
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
        let list = LinkedList::new("first");
        for node in list {
            assert_eq!(*node.borrow().value(), "first".to_string());
        }
    }

    #[test]
    fn unshift_adds_element_to_list() {
        let mut list = LinkedList::new("first");
        list.shift("second");
        list.shift("third");
        let sequence = ["third", "second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(*node.borrow().value(), sequence[index].to_string());
        }
    }
}
