use std::{cell::RefCell, rc::Rc};

/// ListNode is a base struct used in leetcode exercises, is replicated here to
/// make the code possible to be compiled colally.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BTree<T>
where
    T: Clone + Default,
{
    pub val: T,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl<T: Clone + Default> BTree<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        BTree {
            left: None,
            right: None,
            val,
        }
    }
}
pub type TreeNode = BTree<i32>;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ListNode<T>
where
    T: Clone + Default,
{
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Clone + Default> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}
