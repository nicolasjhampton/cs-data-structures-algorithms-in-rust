pub mod linked_list;
pub mod doubly_linked_list;

pub trait RefExt {
    type Reference;
    fn value(&self) -> String;
    fn refer(&self) -> Self::Reference;
    fn next(&self) -> Option<Self::Reference>;
    fn set_next(&mut self, next: Option<Self::Reference>);
}

pub trait DoubleRefExt: RefExt {
    type WeakReference;
    fn weak(&self) -> Self::WeakReference;
    fn prev(&self) -> Option<Self::WeakReference>;
    fn set_prev(&mut self, prev: Option<Self::WeakReference>);
}

pub trait CreateRefExt {
    type Node;
    type Reference;
    fn from_node(node: Self::Node) -> Self::Reference;
}

pub trait CreateDoubleRefExt {
    type Node;
    type Reference;
    type WeakReference;
    fn from_node(node: Self::Node) -> Self::Reference;
}
