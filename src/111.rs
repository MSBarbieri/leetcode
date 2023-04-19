/// Category: algorithms
/// Level: Easy
/// Percent: 44.346924%

/// Given a binary tree, find its minimum depth.
///
/// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
///
/// Note: A leaf is a node with no children.
///
///
/// Example 1:
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 2
///
///
/// Example 2:
///
/// Input: root = [2,null,3,null,4,null,5,null,6]
/// Output: 5
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [0, 10⁵].
/// 	-1000 <= Node.val <= 1000
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
    // pub fn dfs(node: &Node, level: i32) -> i32 {
    //     let n = node.borrow();
    //     match (n.left.as_ref(), n.right.as_ref()) {
    //         (None, None) => level,
    //         (None, Some(r)) => Self::dfs(r, level + 1),
    //         (Some(l), None) => Self::dfs(l, level + 1),
    //         (Some(l), Some(r)) => Self::dfs(l, level + 1).min(Self::dfs(r, level + 1)),
    //     }
    // }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue: VecDeque<(Node, i32)> = VecDeque::from([(root, 1)]);
        let mut depth = 0;
        while let Some(pair) = queue.pop_front() {
            //
            depth = pair.1;
            if let Some(node) = pair.0 {
                let n = node.borrow();
                match (n.left.clone(), n.right.clone()) {
                    (None, None) => break,
                    (None, Some(r)) => queue.push_back((Some(r), depth + 1)),
                    (Some(l), None) => queue.push_back((Some(l), depth + 1)),
                    (Some(l), Some(r)) => {
                        queue.push_back((Some(l), depth + 1));
                        queue.push_back((Some(r), depth + 1));
                    }
                };
            }
        }
        depth
    }
}
/// @lc code=end
struct End;
