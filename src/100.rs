/// Category: algorithms
/// Level: Easy
/// Percent: 58.071083%
/// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
///
/// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
///
///
/// Example 1:
///
/// Input: p = [1,2,3], q = [1,2,3]
/// Output: true
///
///
/// Example 2:
///
/// Input: p = [1,2], q = [1,null,2]
/// Output: false
///
///
/// Example 3:
///
/// Input: p = [1,2,1], q = [1,1,2]
/// Output: false
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in both trees is in the range [0, 100].
/// 	-10⁴ <= Node.val <= 10⁴
///

struct Solution;
use crate::utils::BTree;
type TreeNode = BTree<i32>;
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
type Node = Rc<RefCell<TreeNode>>;
type Root = Option<Node>;
impl Solution {
    pub fn dfs(a: Root, b: Root) -> bool {
        if let Some(node_a) = a {
            if let Some(node_b) = b {
                let left_a = node_a.borrow().left.clone();
                let left_b = node_b.borrow().left.clone();
                let base = Self::dfs(left_a, left_b);
                if !base || node_a.borrow().val != node_b.borrow().val {
                    return false;
                }
                let right_a = node_a.borrow().right.clone();
                let right_b = node_b.borrow().right.clone();
                if !Self::dfs(right_a, right_b) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            if let Some(_) = b {
                return false;
            }
        }
        return true;
    }
    pub fn is_same_tree(p: Option<Node>, q: Option<Node>) -> bool {
        return Self::dfs(p, q);
    }
}
/// @lc code=end
struct End;
