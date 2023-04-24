/// Category: algorithms
/// Level: Easy
/// Percent: 66.95804%

/// Given the root of a binary tree, return the preorder traversal of its nodes' values.
///
///
/// Example 1:
///
/// Input: root = [1,null,2,3]
/// Output: [1,2,3]
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
/// 	The number of nodes in the tree is in the range [0, 100].
/// 	-100 <= Node.val <= 100
///
///
///
/// Follow up: Recursive solution is trivial, could you do it iteratively?

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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //
        let mut vec = Vec::new();
        if let Some(node) = root {
            vec.push(node.borrow().val);
            vec.append(&mut Self::preorder_traversal(node.borrow().left.clone()));
            vec.append(&mut Self::preorder_traversal(node.borrow().right.clone()));
        }

        return vec;
    }
}
/// @lc code=end
struct End;
