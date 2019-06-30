use std::{ rc::Rc };

use super::{ NodeRef, RefExt, CreateRefExt };

#[derive(Debug)]
pub struct Node {
    pub value: String,
    pub next: Option<NodeRef>
}

impl Node {
    pub fn new(value: &str, next: Option<NodeRef>) -> Node {
        Node {
            value: String::from(value),
            next
        }
    }

    pub fn next(&self) -> Option<NodeRef> {
        match &self.next {
            Some(node) => Some(node.refer()),
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
    fn value_returns_string() {
        let node = Node::new("tonia", None);
        assert_eq!("tonia".to_string(), node.value());
    }

    #[test]
    fn next_gives_ref() {
        let mut node = NodeRef::new_ref("tonia", 
            Some(NodeRef::new_ref("nic", 
                Some(NodeRef::new_ref("bill", None))
            ))
        );

        let names = ["tonia", "nic",  "bill"];

        for name in names.iter() {
            assert_eq!(*Rc::clone(&node).value(), name.to_string());
            match Rc::clone(&node).next() {
                Some(new_node) => node = new_node,
                None => break
            }
        }
    }
}
