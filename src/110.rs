/// Category: algorithms
/// Level: Easy
/// Percent: 48.94709%

/// Given a binary tree, determine if it is height-balanced.
///
///
/// Example 1:
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: true
///
///
/// Example 2:
///
/// Input: root = [1,2,2,3,3,null,null,4,4]
/// Output: false
///
///
/// Example 3:
///
/// Input: root = []
/// Output: true
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [0, 5000].
/// 	-10⁴ <= Node.val <= 10⁴
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
    pub fn bfs(left: Node, right: Node, level: i32) -> Result<i32, ()> {
        let ll = if let Some(node) = left {
            let left = Self::bfs(
                node.borrow().left.clone(),
                node.borrow().right.clone(),
                level + 1,
            )?;
            left
        } else {
            level
        };
        let rl = if let Some(node) = right {
            let right = Self::bfs(
                node.borrow().left.clone(),
                node.borrow().right.clone(),
                level + 1,
            )?;
            right
        } else {
            level
        };
        if (ll - rl).abs() <= 1 {
            Ok(ll.max(rl))
        } else {
            return Err(());
        }
        // return left.max(right);
    }
    pub fn is_balanced(root: Node) -> bool {
        println!("#------#");
        if let Some(node) = root {
            return Self::bfs(node.borrow().left.clone(), node.borrow().right.clone(), 1).is_ok();
        }
        return true;
    }
}
/// @lc code=end
struct End;
