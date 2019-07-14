use crate::lists::{ Queue, doubly_linked::DoublyLinkedList as QueueList };

use super::{ RefExt, TreeNode, BinaryNode, node::Node, node_ref::NodeRef };

impl BinaryNode for Node {
    type RefCollection = QueueList;

    #[allow(dead_code)]
    fn insert(&mut self, node: NodeRef) {
        if self.value() >= node.value() {
            if let Some(left) = self.left() {
                left.borrow_mut().insert(node.refer());
            } else {
                self.set_left(Some(node.refer()));
            }
        } else {
            if let Some(right) = self.right() {
                right.borrow_mut().insert(node.refer());
            } else {
                self.set_right(Some(node.refer()));
            }
        }
    }

    #[allow(dead_code)]
    fn depth_walk(&self, coll: &mut QueueList) {
        match self.left() {
            Some(child) => child.borrow().depth_walk(coll),
            None => ()
        };
        coll.push(&self.value());
        match self.right() {
            Some(child) => child.borrow().depth_walk(coll),
            None => ()
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_keeps_binary_sort() {
        let nodes : Vec<usize> = vec![2, 5, 0, 1, 4, 6];
        let node = Node::new("3");
        for i in nodes.iter() {
            node.borrow_mut().insert(Node::new(&i.to_string()));
        }
        let mut coll = QueueList::new();
        node.clone().borrow().depth_walk(&mut coll);
        for (index, value) in coll.enumerate() {
            println!("{:#?}", value);
            assert_eq!(index.to_string(), *value);
        }
    }
}