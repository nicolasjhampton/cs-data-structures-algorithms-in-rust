use std::{ rc::Rc, cell::RefCell };

use super::{ TreeNode, RefExt, node_ref::{ NodeRef } };

#[derive(Debug, Clone)]
pub struct Node {
    data: String,
    left: Option<NodeRef>,
    right: Option<NodeRef>
}

impl Node {
  fn create_ref(node: Node) -> NodeRef {
      Rc::new(RefCell::new(node))
  }

  pub fn new(data: &str) -> NodeRef {
      Node::create_ref(Node {
          data: String::from(data),
          left: None,
          right: None,
      })
  }
}

impl TreeNode for Node {
    type Reference = NodeRef;

    fn value(&self) -> String {
        self.data.clone()
    }

    fn left(&self) -> Option<NodeRef> {
        match &self.left {
            Some(ref child) => Some(child.refer()),
            None => None
        }
    }

    fn set_left(&mut self, left: Option<NodeRef>) {
        self.left = left;
    }

    fn right(&self) -> Option<NodeRef> {
        match &self.right {
            Some(ref child) => Some(child.refer()),
            None => None
        }
    }

    fn set_right(&mut self, right: Option<NodeRef>) {
        self.right = right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_node() {
        let node = Node {
            data: "0".to_string(),
            left: None,
            right: None,
        };

        assert_eq!("0".to_string(), node.data);
    }

    #[test]
    fn new_creates_node() {
        let node = Node::new("1");
        assert_eq!(1.to_string(), node.value());
    }
}