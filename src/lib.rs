use std::rc::Rc;
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

    pub fn get_next(&self) -> Option<Rc<RefCell<LinkedListNode>>> {
        match &self.next {
            Some(node) => Some(Rc::clone(&node)),
            None => None
        }
    }

    pub fn set_next(&mut self, next_node: Rc<RefCell<LinkedListNode>>) {
        self.next = Some(Rc::clone(&next_node));
    }

    pub fn insert(&mut self, new_node: RefCell<LinkedListNode>) {
        match self.get_next() {
            Some(next_node) => {
                new_node.borrow_mut().set_next(Rc::clone(&next_node));
            },
            None => ()
        }
        self.set_next(Rc::new(new_node));
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
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
    fn insert_adds_node_to_end_of_list() {
        let mut node = LinkedListNode::new("tonia");
        node.insert(RefCell::new(LinkedListNode::new("nic")));
        if let Some(next) = node.get_next() {
            assert_eq!("nic", next.borrow().get_value())
        }
    }

    #[test]
    fn insert_adds_node_to_middle_of_list() {
        let mut node = LinkedListNode::new("tonia");
        node.insert(RefCell::new(LinkedListNode::new("nic")));
        node.insert(RefCell::new(LinkedListNode::new("bill")));

        let second_node = Rc::clone(&node.get_next().expect(""));

        assert_eq!(*Rc::clone(&second_node).borrow().get_value(), "bill".to_string());

        let third_node = Rc::clone(&second_node).borrow().get_next().expect("");

        assert_eq!(*Rc::clone(&third_node).borrow().get_value(), "nic".to_string());
    }

    #[test]
    fn next_gives_ref() {
        let node = LinkedListNode {
            value: String::from("tonia"),
            next: Some(Rc::new(RefCell::new(LinkedListNode {
                value: String::from("nic"),
                next: None
            })))
        };

        match node.get_next() {
            Some(ref next_node) => assert_eq!("nic", next_node.borrow().get_value()),
            None => assert_eq!(1, 2)
        }
    }
}
