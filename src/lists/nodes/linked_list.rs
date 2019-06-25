use std::{ cell::RefCell, rc::Rc };

pub struct Node {
    pub value: String,
    pub next: Option<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(value: &str, next: Option<Rc<RefCell<Node>>>) -> Node {
        Node {
            value: String::from(value),
            next
        }
    }

    pub fn next(&self) -> Option<Rc<RefCell<Node>>> {
        match &self.next {
            Some(node) => Some(Rc::clone(&node)),
            None => None
        }
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node {
            value: String::from("tonia"),
            next: None
        };

        assert_eq!("tonia", node.value);
    }

    #[test]
    fn new_creates_node() {
        let node = Node::new("tonia", None);
        assert_eq!("tonia", node.value);
    }

    #[test]
    fn next_gives_ref() {
        let mut node = Rc::new(RefCell::new(
            Node::new("tonia", Some(
                Rc::new(RefCell::new(
                    Node::new("nic", Some(
                        Rc::new(RefCell::new(
                            Node::new("bill", None)
                        ))
                    ))
                ))
            ))
        ));

        let names = ["tonia", "nic",  "bill"];

        for name in names.iter() {
            assert_eq!(*Rc::clone(&node).borrow().value(), name.to_string());
            match Rc::clone(&node).borrow().next() {
                Some(new_node) => node = new_node,
                None => break
            }
        }
    }
}
