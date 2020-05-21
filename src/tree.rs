use crate::html::Function;
use alloc::rc::{Rc, Weak};
use alloc::vec::Vec;
use core::cell::RefCell;

#[derive(Clone)]
pub struct Tree {
    root: RcTreeNode,
}

pub type RcTreeNode = Rc<TreeNodeCell>;
type TreeNodeCell = RefCell<TreeNode>;

pub struct TreeNode {
    value: Rc<RefCell<Function>>,
    parent: Weak<TreeNodeCell>,
    children: Vec<RcTreeNode>,
}

impl Tree {
    pub fn new(value: Function) -> Self {
        Self {
            root: Rc::new(TreeNodeCell::new(TreeNode::new(value))),
        }
    }

    pub fn insert(&self, node: RcTreeNode) {
        node.borrow_mut().parent(Rc::downgrade(&self.root));
        self.root.borrow_mut().children.push(node);
    }

    pub fn parent(&self, parent: Weak<TreeNodeCell>) {
        self.root.borrow_mut().parent = parent;
    }

    pub fn value(&self) -> Rc<RefCell<Function>> {
        self.root.borrow().value()
    }

    pub fn root(&self) -> RcTreeNode {
        Rc::clone(&self.root)
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new(Function::new())
    }
}

impl TreeNode {
    fn new(value: Function) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            parent: Weak::new(),
            children: Vec::new(),
        }
    }

    pub fn value(&self) -> Rc<RefCell<Function>> {
        Rc::clone(&self.value)
    }

    fn parent(&mut self, parent: Weak<TreeNodeCell>) {
        self.parent = parent;
    }

    pub fn children(&self) -> &Vec<RcTreeNode> {
        &self.children
    }
}
