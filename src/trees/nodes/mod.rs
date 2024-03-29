pub mod node;
pub mod node_ref;
pub mod binary_tree;

pub trait TreeNode {
  type Reference;
  fn value(&self) -> String;
  fn left(&self) -> Option<Self::Reference>;
  fn set_left(&mut self, next: Option<Self::Reference>);
  fn right(&self) -> Option<Self::Reference>;
  fn set_right(&mut self, next: Option<Self::Reference>);
}

pub trait BinaryNode: TreeNode {
  type RefCollection;
  fn insert(&mut self, node: Self::Reference);
  fn depth_walk(&self, coll: &mut Self::RefCollection);
}

pub trait RefExt {
    type Reference;
    fn value(&self) -> String;
    fn refer(&self) -> Self::Reference;
    fn left(&self) -> Option<Self::Reference>;
    fn set_left(&mut self, next: Option<Self::Reference>);
    fn right(&self) -> Option<Self::Reference>;
    fn set_right(&mut self, next: Option<Self::Reference>);
}

// pub trait CreateRefExt {
//     type Node;
//     type Reference;
//     fn from_node(node: Self::Node) -> Self::Reference;
// }