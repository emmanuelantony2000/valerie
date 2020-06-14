// use crate::Function;

// use intrusive_collections::intrusive_adapter;
// use intrusive_collections::{KeyAdapter, RBTree, RBTreeLink};
// use alloc::sync::Arc;
// use core::ops::{Deref, DerefMut};

// intrusive_adapter!(pub Adapter = Arc<TreeElement>: TreeElement { link: RBTreeLink });
// impl<'a> KeyAdapter<'a> for Adapter {
//     type Key = usize;
//     fn get_key(&self, x: &'a TreeElement) -> Self::Key {
//         x.id
//     }
// }

// pub struct Tree {
//     tree: RBTree<Adapter>,
//     count: usize,
// }

// impl Tree {
//     pub fn new() -> Self {
//         Self {
//             tree: RBTree::new(Adapter::new()),
//             count: 0,
//         }
//     }

//     pub fn push(&mut self, child: Function) {
//         self.tree.insert(Arc::new(TreeElement {
//             link: RBTreeLink::default(),
            
//                 id: {
//                     if let Some(x) = self.tree.back().get() {
//                         x.id + 1
//                     } else {
//                         0
//                     }
//                 },
//                 function: child,
            
//         }));
//     }
// }

// impl Deref for Tree {
//     type Target = RBTree<Adapter>;

//     fn deref(&self) -> &Self::Target {
//         &self.tree
//     }
// }

// impl DerefMut for Tree {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.tree
//     }
// }

// // #[derive(Default)]
// pub struct TreeElement {
//     link: RBTreeLink,
//     id: usize,
//     function: Function,
// }

// // #[derive(Default)]
// // struct TreeNode {
// //     id: usize,
// //     element: Function,
// // }

// // enum TreeNodeType {
// //     Element(Function, Tree),
// //     Text(Function),
// // }

// // impl Default for TreeNodeType {
// //     fn default() -> Self {
// //         Self::Text(Function::default())
// //     }
// // }
