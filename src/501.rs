/// Category: algorithms
/// Level: Easy
/// Percent: 49.255608%

/// Given the root of a binary search tree (BST) with duplicates, return all the mode(s) (i.e., the most frequently occurred element) in it.
///
/// If the tree has more than one mode, return them in any order.
///
/// Assume a BST is defined as follows:
///
///
/// 	The left subtree of a node contains only nodes with keys less than or equal to the node's key.
/// 	The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
/// 	Both the left and right subtrees must also be binary search trees.
///
///
///
/// Example 1:
///
/// Input: root = [1,null,2,2]
/// Output: [2]
///
///
/// Example 2:
///
/// Input: root = [0]
/// Output: [0]
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in the tree is in the range [1, 10⁴].
/// 	-10⁵ <= Node.val <= 10⁵
///
///
///
/// Follow up: Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).

struct Solution;
use crate::utils::BTree;
type TreeNode = BTree<i32>;
use std::borrow::Borrow;
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

impl Default for TreeNode {
    fn default() -> Self {
        Self {
            val: 0,
            left: None,
            right: None,
        }
    }
}
impl TreeNode {
    pub fn add_to_map(self, map: &mut HashMap<i32, i32>) -> i32 {
        //
        let val = self.val;
        let mut result = 1;
        if map.contains_key(&val) {
            let v = map.get_mut(&val).unwrap();
            *v += 1;
            result = *v;
        } else {
            map.insert(val, 1);
        }
        if let Some(left) = self.left {
            result = result.max(left.take().add_to_map(map));
        }
        if let Some(right) = self.right {
            result = result.max(right.take().add_to_map(map));
        }
        result
    }
}

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let max = if let Some(r) = root {
            r.take().add_to_map(&mut map)
        } else {
            0
        };

        let mut result = Vec::new();
        // println!("{:?}, {:?}", map, max);
        for (k, v) in map.iter() {
            if *v == max {
                result.push(*k);
            }
        }
        return result;
    }
}
/// @lc code=end
struct End;
