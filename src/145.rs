/// Category: algorithms
/// Level: Easy
/// Percent: 67.42064%

/// Given the root of a binary tree, return the postorder traversal of its nodes' values.
///
///
/// Example 1:
///
/// Input: root = [1,null,2,3]
/// Output: [3,2,1]
///
///
/// Example 2:
///
/// Input: root = []
/// Output: []
///
///
/// Example 3:
///
/// Input: root = [1]
/// Output: [1]
///
///
///
/// Constraints:
///
///
/// 	The number of the nodes in the tree is in the range [0, 100].
/// 	-100 <= Node.val <= 100
///
///
///
/// Follow up: Recursive solution is trivial, could you do it iteratively?
pub struct Solution;
use crate::utils::BTree as BT;
type TreeNode = BT<i32>;
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
    pub fn _postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            Solution::_postorder_traversal(node.borrow().left.clone(), result);
            Solution::_postorder_traversal(node.borrow().right.clone(), result);
            result.push(node.borrow().val);
        }
    }
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Solution::_postorder_traversal(root, &mut result);
        result
    }
}
/// @lc code=end
struct End;
