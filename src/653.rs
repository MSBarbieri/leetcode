/// Category: algorithms
/// Level: Easy
/// Percent: 61.032516%
/// Given the root of a binary search tree and an integer k, return true if there exist two elements in the BST such that their sum is equal to k, or false otherwise.
///
///
/// Example 1:
///
/// Input: root = [5,3,6,2,4,null,7], k = 9
/// Output: true
///
///
/// Example 2:
///
/// Input: root = [5,3,6,2,4,null,7], k = 28
/// Output: false
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [1, 10⁴].
/// 	-10⁴ <= Node.val <= 10⁴
/// 	root is guaranteed to be a valid binary search tree.
/// 	-10⁵ <= k <= 10⁵
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
impl Solution {
    pub fn inorder_traversal(cur_node: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        if let Some(node) = cur_node {
            Self::inorder_traversal(node.borrow().left.clone(), list);
            list.push(node.borrow().val);
            Self::inorder_traversal(node.borrow().right.clone(), list);
        }
    }
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut vec = Vec::new();
        Self::inorder_traversal(root, &mut vec);

        let mut start = 0;
        let mut end = vec.len() - 1;
        while start < end {
            let sv = vec.get(start).unwrap();
            let ev = vec.get(end).unwrap();
            if sv + ev == k {
                return true;
            } else if sv + ev > k {
                end -= 1;
            } else {
                start += 1;
            }
        }
        false
    }
}
/// @lc code=end
struct End;
