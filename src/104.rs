/// Category: algorithms
/// Level: Easy
/// Percent: 73.86303%
/// Given the root of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
///
///
/// Example 1:
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 3
///
///
/// Example 2:
///
/// Input: root = [1,null,2]
/// Output: 2
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [0, 10⁴].
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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn dfs(cur: Node, mut level: i32) -> i32 {
        if let Some(node) = cur {
            level += 1;
            return Self::dfs(node.borrow().left.clone(), level)
                .max(Self::dfs(node.borrow().right.clone(), level));
        }
        return level;
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Self::dfs(root, 0);
    }
}
/// @lc code=end
struct End;
