/// Category: algorithms
/// Level: Easy
/// Percent: 77.62739%
/// You are given the root of a binary search tree (BST) and an integer val.
///
/// Find the node in the BST that the node's value equals val and return the subtree rooted with that node. If such a node does not exist, return null.
///
///
/// Example 1:
///
/// Input: root = [4,2,7,1,3], val = 2
/// Output: [2,1,3]
///
///
/// Example 2:
///
/// Input: root = [4,2,7,1,3], val = 5
/// Output: []
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [1, 5000].
/// 	1 <= Node.val <= 10⁷
/// 	root is a binary search tree.
/// 	1 <= val <= 10⁷
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
    pub fn search_bst(root: Root, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(node) => {
                let r_val = node.borrow().val;
                if r_val == val {
                    return Some(node);
                }
                if r_val > val {
                    return Self::search_bst(node.borrow().left.clone(), val);
                } else {
                    return Self::search_bst(node.borrow().right.clone(), val);
                }
            }
        }
    }
}
/// @lc code=end
struct End;
