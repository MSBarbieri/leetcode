/// Category: algorithms
/// Level: Easy
/// Percent: 69.76905%

/// Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.
///
///
/// Example 1:
///
/// Input: nums = [-10,-3,0,5,9]
/// Output: [0,-3,9,-10,null,5]
/// Explanation: [0,-10,5,null,-3,null,9] is also accepted:
///
///
///
/// Example 2:
///
/// Input: nums = [1,3]
/// Output: [3,1]
/// Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁴
/// 	-10⁴ <= nums[i] <= 10⁴
/// 	nums is sorted in a strictly increasing order.
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
fn inner(cur: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if cur.len() == 0 {
        return None;
    }
    if cur.len() == 1 {
        let node = Some(Rc::new(RefCell::new(TreeNode::new(*cur.get(0).unwrap()))));
        return node;
    }

    let length = cur.len();

    let index = length / 2;
    let lower = &cur[0..index];
    let higher = &cur[index..cur.len() - 1];

    let mut result_node = TreeNode::new(*cur.get(index).unwrap());
    if lower.len() > 0 {
        result_node.left = inner(&cur[0..index]);
    }

    if higher.len() > 0 {
        result_node.right = inner(&cur[index+1..cur.len()]);
    }
    Some(Rc::new(RefCell::new(result_node)))
}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        inner(&nums)
    }
}
/// @lc code=end
struct End;
