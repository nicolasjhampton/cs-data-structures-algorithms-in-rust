use std::{ cell::RefCell, rc::{ Rc, Weak }};

use super::nodes::doubly_linked_list::{ DoubleNode, DoubleNodeRef, WeakDoubleNodeRef };

use super::nodes::{ RefExt, DoubleRefExt, CreateDoubleRefExt };

use super::{ Stack, Queue };

#[derive(Debug)]
pub struct DoublyLinkedList {
    pub head: Option<Rc<RefCell<DoubleNode>>>,
    pub tail: WeakDoubleNodeRef,
}

impl DoublyLinkedList {
    #[allow(dead_code)]
    fn new(start: &str) -> DoublyLinkedList {
        let node = DoubleNode::new(start, None, None);
        DoublyLinkedList {
            head: Some(node.refer()),
            tail: node.weak()
        }
    }
}

impl Stack for DoublyLinkedList {

    type Reference = DoubleNodeRef;

    fn set_head(&mut self, node: Option<DoubleNodeRef>) {
        if let Some(head) = &node {
            head.refer().set_prev(None);
        }
        println!("{:#?}", &node);
        self.head = node;
    }

    fn head(&self) -> Option<DoubleNodeRef> {
        match self.head {
            Some(ref node) => Some(node.refer()),
            None => None
        }
    }

    fn unshift(&mut self, data: &str) {
        if let Some(current) = self.head() {
            self.set_head(Some(DoubleNode::new(data, Some(current), None)));
        };
    }

    fn shift(&mut self) -> Option<String> {
        let current_value = match self.head() {
            Some(ref node) => Some(node.value()),
            None => None
        };
        let next = match self.head() {
            Some(ref node) => node.next(),
            None => None
        };
        match next {
            Some(ref node) => self.set_head(Some(node.refer())),
            None => self.set_head(None)
        };
        current_value
    }
}

impl Queue for DoublyLinkedList {
    type WeakReference = WeakDoubleNodeRef;

    fn tail(&self) -> Option<DoubleNodeRef> {
        match self.tail.upgrade() {
            Some(node) => Some(node),
            None => None
        }
    }

    fn set_tail(&mut self, node: WeakDoubleNodeRef) {
        if let Some(tail) = &node.upgrade() {
            tail.refer().set_next(None);
        }
        self.tail = node;
    }

    fn pop(&mut self) -> Option<String> {
        // current_value == "value"
        let current_value = match self.tail() {
            Some(ref node) => Some(node.value()),
            None => None
        };
        // 
        let prev = match self.tail() {
            Some(ref node) => node.prev(),
            None => None
        };
        match prev {
            Some(ref node) => {
                match node.upgrade() {
                    Some(deep_node) => self.set_tail(deep_node.weak()),
                    None => ()
                }
            },
            None => self.set_tail(Weak::new())
        };
        current_value
    }

    fn push(&mut self, data: &str) {
        if let Some(current) = self.tail() {
            let tail = DoubleNode::new(data, None, Some(current.weak()));
            self.set_tail(tail.weak());
        };
    }
}

impl Iterator for DoublyLinkedList {
    type Item = String;

    fn next(&mut self) -> Option<String>  {
        self.shift()
    }
}

impl DoubleEndedIterator for DoublyLinkedList {
    fn next_back(&mut self) -> Option<String>  {
        self.pop()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubly_linked_list_is_iterator() {
        let node = DoubleNode::new("first", None, None);
        let list = DoublyLinkedList {
            head: Some(node.refer()),
            tail: node.weak()
        };
        for node in list {
            assert_eq!(node, "first".to_string());
        }
    }

    #[test]
    fn doubly_linked_list_is_double_ended_iterator() {
        let node = DoubleNode::new("first", None, None);
        let list = DoublyLinkedList {
            head: Some(node.refer()),
            tail: node.weak()
        };
        for node in list.rev() {
            assert_eq!(node, "first".to_string());
        }
    }

    #[test]
    fn new_creates_doubly_linked_list() {
        let list = DoublyLinkedList::new("first");
        for node in list {
            assert_eq!(node, "first".to_string());
        }
    }

    #[test]
    fn unshift_adds_element_to_doubly_list() {
        let mut list = DoublyLinkedList::new("first");
        list.unshift("second");
        list.unshift("third");
        let sequence = ["third", "second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }

    #[test]
    fn shift_removes_element_from_doubly_list() {
        let mut list = DoublyLinkedList::new("first");
        list.unshift("second");
        list.unshift("third");
        let third = list.shift();
        assert_eq!(third.unwrap(), "third".to_string());
        let sequence = ["second", "first"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }

    #[test]
    fn push_adds_element_to_doubly_list() {
        let mut list = DoublyLinkedList::new("first");
        list.push("second");
        list.push("third");
        let sequence = ["first", "second", "third"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }

    #[test]
    fn pop_removes_element_from_doubly_list() {
        let mut list = DoublyLinkedList::new("first");
        list.push("second");
        list.push("third");
        let third = list.pop();
        assert_eq!(third.unwrap(), "third".to_string());
        let sequence = ["first", "second"];
        for (index, node) in list.enumerate() {
            assert_eq!(node, sequence[index].to_string());
        }
    }
}
