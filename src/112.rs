/// Category: algorithms
/// Level: Easy
/// Percent: 48.19029%

/// Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
///
/// A leaf is a node with no children.
///
///
/// Example 1:
///
/// Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
/// Output: true
/// Explanation: The root-to-leaf path with the target sum is shown.
///
///
/// Example 2:
///
/// Input: root = [1,2,3], targetSum = 5
/// Output: false
/// Explanation: There two root-to-leaf paths in the tree:
/// (1 --> 2): The sum is 3.
/// (1 --> 3): The sum is 4.
/// There is no root-to-leaf path with sum = 5.
///
///
/// Example 3:
///
/// Input: root = [], targetSum = 0
/// Output: false
/// Explanation: Since the tree is empty, there are no root-to-leaf paths.
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [0, 5000].
/// 	-1000 <= Node.val <= 1000
/// 	-1000 <= targetSum <= 1000
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
use std::collections::VecDeque;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn has_path_sum(root: Node, target_sum: i32) -> bool {
        println!("------");
        match root.clone() {
            None => false,
            Some(ref _root) => {
                let mut queue: VecDeque<(Node, i32)> = VecDeque::from([(root, _root.borrow().val)]);
                while let Some(pair) = queue.pop_front() {
                    let cur_val = pair.1;
                    if let Some(node) = pair.0 {
                        let n = node.borrow();
                        match (n.left.clone(), n.right.clone()) {
                            (None, None) => {
                                if target_sum == cur_val {
                                    return true;
                                }
                            }
                            (None, Some(r)) => {
                                queue.push_back((Some(r.clone()), cur_val + r.borrow().val))
                            }
                            (Some(l), None) => {
                                queue.push_back((Some(l.clone()), cur_val + l.borrow().val))
                            }
                            (Some(l), Some(r)) => {
                                queue.push_back((Some(r.clone()), cur_val + r.borrow().val));
                                queue.push_back((Some(l.clone()), cur_val + l.borrow().val));
                            }
                        }
                    }
                }
                false
            }
        }
    }
}
/// @lc code=end
struct End;
