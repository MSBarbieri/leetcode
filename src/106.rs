/// Category: algorithms
/// Level: Medium
/// Percent: 60.065693%

/// Given two integer arrays inorder and postorder where inorder is the inorder traversal of a binary tree and postorder is the postorder traversal of the same tree, construct and return the binary tree.
///
///
/// Example 1:
///
/// Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
/// Output: [3,9,20,null,null,15,7]
///
///
/// Example 2:
///
/// Input: inorder = [-1], postorder = [-1]
/// Output: [-1]
///
///
///
/// Constraints:
///
///
/// 	1 <= inorder.length <= 3000
/// 	postorder.length == inorder.length
/// 	-3000 <= inorder[i], postorder[i] <= 3000
/// 	inorder and postorder consist of unique values.
/// 	Each value of postorder also appears in inorder.
/// 	inorder is guaranteed to be the inorder traversal of the tree.
/// 	postorder is guaranteed to be the postorder traversal of the tree.
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
    pub fn res(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() || inorder.is_empty() {
            return None;
        }
        let nv = postorder.last().unwrap();

        let ilen = inorder.len();
        let mut i = 0;
        while i < ilen - 1 && inorder[i] != *nv {
            i += 1;
        }
        let mut node = TreeNode::new(*nv);

        // Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
        // Output: [3,9,20,null,null,15,7]
        node.left = Self::res(&inorder[0..i], &postorder[0..i]);
        node.right = Self::res(&inorder[i + 1..], &postorder[i..postorder.len() - 1]);

        Some(Rc::new(RefCell::new(node)))
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::res(&inorder, &postorder);
    }
}
/// @lc code=end
struct End;
