/// Category: algorithms
/// Level: Easy
/// Percent: 74.80244%

/// Given the root of a binary tree, invert the tree, and return its root.
///
///
/// Example 1:
///
/// Input: root = [4,2,7,1,3,6,9]
/// Output: [4,7,2,9,6,3,1]
///
///
/// Example 2:
///
/// Input: root = [2,1,3]
/// Output: [2,3,1]
///
///
/// Example 3:
///
/// Input: root = []
/// Output: []
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [0, 100].
/// 	-100 <= Node.val <= 100
///

struct Solution;
use crate::utils::TreeNode;
/// @lc code=start
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
impl Solution {
    pub fn invert_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(mut node) = root {
            let mut n = node.take();
            let left = n.left;
            let right = n.right;
            let r = Self::invert_tree(left.clone());
            let l = Self::invert_tree(right.clone());
            n.left = l;
            n.right = r;
            return Some(Rc::new(RefCell::new(n)));
        }
        return root;
    }
}
/// @lc code=end
struct End;
