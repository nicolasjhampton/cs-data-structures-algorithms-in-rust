use std::rc::Rc;
use std::cell::RefCell;

pub struct LinkedListNode {
    value: String,
    next: Option<Rc<RefCell<LinkedListNode>>>
}

impl LinkedListNode {
    pub fn get_next(&self) -> Option<Rc<RefCell<LinkedListNode>>> {
        match &self.next {
            Some(node) => Some(node.clone()),
            None => None
        }
    }

    pub fn set_next(&mut self, next_node: RefCell<LinkedListNode>) {
        self.next = Some(Rc::new(next_node));
    }

    pub fn insert(&mut self, new_node: &mut LinkedListNode) {
        if let Some(next_node) = self.get_next() {
            new_node.next = Some(next_node.clone());
        }
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
            next: Some(Rc::new(RefCell::new(LinkedListNode {
                value: String::from("nic"),
                next: None
            })))
        };

        assert_eq!("tonia", node.value);
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
