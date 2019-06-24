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
            assert_eq!(*Rc::clone(&node).borrow().get_value(), name.to_string());
            match Rc::clone(&node).borrow().get_next() {
                Some(new_node) => node = new_node,
                None => break
            }
        }
        // match node.get_next() {
        //     Some(ref next_node) => assert_eq!("nic", next_node.borrow().get_value()),
        //     None => assert_eq!(1, 2)
        // }
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
        let mut node = Rc::new(RefCell::new(LinkedListNode::new("tonia")));
        node.borrow_mut().insert(RefCell::new(LinkedListNode::new("nic")));
        node.borrow_mut().insert(RefCell::new(LinkedListNode::new("bill")));

        let names = ["tonia", "bill", "nic"];

        for name in names.iter() {
            assert_eq!(*Rc::clone(&node).borrow().get_value(), name.to_string());
            match Rc::clone(&node).borrow().get_next() {
                Some(new_node) => node = new_node,
                None => break
            }
        }
    }
}
