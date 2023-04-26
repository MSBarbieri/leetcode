/// Category: algorithms
/// Level: Medium
/// Percent: 60.275154%

/// Given the head of a singly linked list where elements are sorted in ascending order, convert it to a height-balanced binary search tree.
///
///
/// Example 1:
///
/// Input: head = [-10,-3,0,5,9]
/// Output: [0,-3,9,-10,null,5]
/// Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the shown height balanced BST.
///
///
/// Example 2:
///
/// Input: head = []
/// Output: []
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in head is in the range [0, 2 * 10⁴].
/// 	-10⁵ <= Node.val <= 10⁵
///

struct Solution;
use crate::utils::{ListNode as RawList, TreeNode};
type ListNode = RawList<i32>;
/// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
type TNode = Option<Rc<RefCell<TreeNode>>>;
type LNode = Option<Box<ListNode>>;
impl Solution {
    pub fn transform(mut head: LNode) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }

        return vec;
    }
    pub fn traverse_vec(vec: &[i32]) -> TNode {
        let len = vec.len();
        if len == 0 {
            return None;
        }
        if len == 1 {
            let node = TreeNode::new(vec[0]);
            return Some(Rc::new(RefCell::new(node)));
        }
        let mid = len / 2;
        let mut node = TreeNode::new(vec[mid]);
        node.left = Self::traverse_vec(&vec[0..mid]);
        node.right = Self::traverse_vec(&vec[mid + 1..len]);
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        //
        let vec = Self::transform(head);

        return Self::traverse_vec(&vec);
    }
}
/// @lc code=end
struct End;
