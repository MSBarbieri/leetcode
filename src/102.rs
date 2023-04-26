/// Category: algorithms
/// Level: Medium
/// Percent: 64.46631%

/// Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
///
///
/// Example 1:
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [[3],[9,20],[15,7]]
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
use std::collections::HashMap;
use std::rc::Rc;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn tarversal(cur: Node, map: &mut HashMap<usize, Vec<i32>>, level: usize) -> usize {
        if let Some(node) = cur {
            let list = map.get_mut(&level);
            if let Some(l) = list {
                l.push(node.borrow().val);
            } else {
                let l = vec![node.borrow().val];
                map.insert(level, l);
            }
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();
            let max_size = Self::tarversal(left, map, level + 1);
            return max_size.max(Self::tarversal(right, map, level + 1));
        }
        return level - 1;
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map: HashMap<usize, Vec<i32>> = HashMap::new();
        let level = Self::tarversal(root, &mut map, 1);
        let mut result = vec![];
        for i in 1..=level {
            result.push(map.get(&i).unwrap().clone());
        }
        result
    }
}
/// @lc code=end
struct End;
