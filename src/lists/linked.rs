use std::{ cell::RefCell, rc::{ Rc }};

use super::nodes::linked_list::Node;

use super::Stack;

#[derive(Debug)]
pub struct LinkedList {
    pub curr: Option<Rc<RefCell<Node>>>
}

impl LinkedList {
    #[allow(dead_code)]
    fn new(start: &str) -> LinkedList {
        let tail = Node::new(start, None);
        LinkedList {
            curr: Some(tail)
        }
    }
}

impl Stack for LinkedList {
    fn set_curr(&mut self, node: Option<Rc<RefCell<Node>>>) {
        self.curr = node;
    }

    fn current(&self) -> Option<Rc<RefCell<Node>>> {
        match self.curr {
            Some(ref curr) => Some(Rc::clone(curr)),
            None => None
        }
    }
}

impl Iterator for LinkedList {
    type Item = String;

    fn next(&mut self) -> Option<String>  {
        self.shift()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_is_iterator() {
        let node = Node::new("first", None);
        let list = LinkedList {
            curr: Some(node)
        };
        for node in list {
            assert_eq!(node, "first".to_string());
        }
    }

    #[test]
    fn new_creates_linked_list() {
        let list = LinkedList::new("first");
        for node in list {
            assert_eq!(node, "first".to_string());
        }
    }

    #[test]
    fn unshift_adds_element_to_list() {
        let mut list = LinkedList::new("first");
        list.unshift("second");
        list.unshift("third");
        let sequence = ["third", "second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }

    #[test]
    fn remove_removes_element_from_list() {
        let mut list = LinkedList::new("first");
        list.unshift("second");
        list.unshift("third");
        let third = list.shift();
        assert_eq!(third.unwrap(), "third".to_string());
        let sequence = ["second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }
}
