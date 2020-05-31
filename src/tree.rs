use crate::Function;

use intrusive_collections::intrusive_adapter;
use intrusive_collections::{KeyAdapter, RBTree, RBTreeLink};
use alloc::sync::Arc;
use core::ops::{Deref, DerefMut};

intrusive_adapter!(Adapter = Arc<TreeNode>: TreeNode { link: RBTreeLink });
impl<'a> KeyAdapter<'a> for Adapter {
    type Key = usize;
    fn get_key(&self, x: &'a TreeNode) -> Self::Key {
        x.value.id
    }
}

#[derive()]
struct Tree(RBTree<Adapter>);

impl Deref for Tree {
    type Target = RBTree<Adapter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
struct TreeNode {
    link: RBTreeLink,
    value: TreeElement,
}

#[derive(Default)]
struct TreeElement {
    id: usize,
    element: Either,
}

enum Either {
    Tree(Tree),
    Value(Function),
}

impl Default for Either {
    fn default() -> Self {
        Self::Value(Function::default())
    }
}
