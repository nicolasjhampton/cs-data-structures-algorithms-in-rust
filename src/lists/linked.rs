use std::{ cell::RefCell, rc::{ Rc }};

use super::nodes::linked_list::{ Node, NodeRef };

use super::Stack;

#[derive(Debug)]
pub struct LinkedList {
    pub curr: Option<NodeRef>
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

    type Reference = NodeRef;

    fn set_curr(&mut self, node: Option<NodeRef>) {
        self.curr = node;
    }

    fn current(&self) -> Option<NodeRef> {
        match self.curr {
            Some(ref curr) => Some(Rc::clone(curr)),
            None => None
        }
    }

    fn unshift(&mut self, data: &str) {
        if let Some(current) = self.current() {
            self.set_curr(Some(Node::new(data, Some(current))));
        };
    }

    fn shift(&mut self) -> Option<String> {
        let current_value = match self.current() {
            Some(ref node) => Some(node.borrow().value()),
            None => None
        };
        let next = match self.current() {
            Some(ref node) => node.borrow().next(),
            None => None
        };
        match next {
            Some(ref node) => self.set_curr(Some(Rc::clone(node))),
            None => self.set_curr(None)
        };
        current_value
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
