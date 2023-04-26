/// Category: algorithms
/// Level: Medium
/// Percent: 56.96454%

/// Given the root of a binary tree, return the zigzag level order traversal of its nodes' values. (i.e., from left to right, then right to left for the next level and alternate between).
///
///
/// Example 1:
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [[3],[20,9],[15,7]]
///
///
/// Example 2:
///
/// Input: root = [1]
/// Output: [[1]]
///
///
/// Example 3:
///
/// Input: root = []
/// Output: []
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [0, 2000].
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
use std::collections::HashMap;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn traverse(cur: Node, map: &mut HashMap<usize, Vec<i32>>, level: usize) -> usize {
        if let Some(node) = cur {
            let val = node.borrow().val;
            let list = map.get_mut(&level);
            if let Some(l) = list {
                l.push(val);
            } else {
                let l = vec![val];
                map.insert(level, l);
            }

            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            let max_number = Self::traverse(left, map, level + 1);
            return max_number.max(Self::traverse(right, map, level + 1));
        }
        return level - 1;
    }
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let size = Self::traverse(root, &mut map, 1);
        let mut result = vec![];
        for i in 1..=size {
            let mut list = map.get(&i).unwrap().clone();
            if i % 2 == 0 {
                list = list.into_iter().rev().collect();
            }
            result.push(list);
        }
        return result;
    }
}
/// @lc code=end
struct End;
