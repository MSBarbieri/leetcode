/// Category: algorithms
/// Level: Medium
/// Percent: 61.628498%

/// Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
///
///
/// Example 1:
///
/// Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
/// Output: [3,9,20,null,null,15,7]
///
///
/// Example 2:
///
/// Input: preorder = [-1], inorder = [-1]
/// Output: [-1]
///
///
///
/// Constraints:
///
///
/// 	1 <= preorder.length <= 3000
/// 	inorder.length == preorder.length
/// 	-3000 <= preorder[i], inorder[i] <= 3000
/// 	preorder and inorder consist of unique values.
/// 	Each value of inorder also appears in preorder.
/// 	preorder is guaranteed to be the preorder traversal of the tree.
/// 	inorder is guaranteed to be the inorder traversal of the tree.
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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn _build_tree(preorder: &[i32], inorder: &[i32]) -> Node {
        let len = inorder.len();
        if len == 0 {
            // println!("none: {:?}-{:?}", preorder, inorder);
            return None;
        }

        let node = preorder[0];
        let mut i = 0;
        while i < len && inorder[i] != node {
            i += 1;
        }
        // println!("{:?}-{:?},{i}-{node}", preorder, inorder);
        let mut result = TreeNode::new(node);
        result.left = Self::_build_tree(&preorder[1..len], &inorder[0..i]);
        result.right = Self::_build_tree(&preorder[i + 1..len], &inorder[i + 1..len]);
        return Some(Rc::new(RefCell::new(result)));
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // println!("-----");
        Self::_build_tree(&preorder, &inorder)
    }
}
/// @lc code=end
struct End;
