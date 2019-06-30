use std::{ rc::Rc, cell::RefCell };

use super::{ RefExt, CreateRefExt };

pub type NodeRef = Rc<RefCell<Node>>;

impl CreateRefExt for NodeRef {
    type Node = Node;
    type Reference = NodeRef;

    fn from_node(node: Node) -> NodeRef {
        Rc::new(RefCell::new(node))
    }
}

impl RefExt for NodeRef {
    type Reference = NodeRef;

    fn next(&self) -> Option<NodeRef> {
        self.borrow().next()
    }

    fn set_next(&mut self, next: Option<NodeRef>) {
        self.borrow_mut().next = next;
    }

    fn value(&self) -> String {
        self.borrow().value()
    }

    fn refer(&self) -> NodeRef {
        Rc::clone(self)
    }
}

#[derive(Debug)]
pub struct Node {
    pub value: String,
    pub next: Option<NodeRef>
}

impl Node {
    pub fn new(value: &str, next: Option<NodeRef>) -> NodeRef {
        NodeRef::from_node(Node {
            value: String::from(value),
            next
        })
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
        assert_eq!("tonia", node.value());
    }

    #[test]
    fn value_returns_string() {
        let node = Node::new("tonia", None);
        assert_eq!("tonia".to_string(), node.value());
    }

    #[test]
    fn next_gives_ref() {
        let mut node = Node::new("tonia", 
            Some(Node::new("nic", 
                Some(Node::new("bill", None))
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
