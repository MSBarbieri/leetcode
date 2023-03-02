/// Category: algorithms
/// Level: Medium
/// Percent: 40.118675%

/// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
///
/// Example 1:
///
/// Input: l1 = [2,4,3], l2 = [5,6,4]
/// Output: [7,0,8]
/// Explanation: 342 + 465 = 807.
///
///
/// Example 2:
///
/// Input: l1 = [0], l2 = [0]
/// Output: [0]
///
///
/// Example 3:
///
/// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
/// Output: [8,9,9,9,0,0,0,1]
///
///
///
/// Constraints:
///
///
/// 	The number of nodes in each linked list is in the range [1, 100].
/// 	0 <= Node.val <= 9
/// 	It is guaranteed that the list represents a number that does not have leading zeros.
///

struct Solution;
use crate::ListNode as LL;
type ListNode = LL<i32>;
/// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn push_val(cur: &mut Option<Box<ListNode>>, val: i32) -> &mut Option<Box<ListNode>> {
        if let Some(node) = cur {
            return Solution::push_val(&mut node.next, val);
        } else {
            *cur = Some(Box::new(ListNode::new(val)));
            return cur;
        }
    }
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = ListNode::new(0);
        let mut rest = 0;
        let mut foo = &mut result.next;
        while l1.is_some() || l2.is_some() || rest > 0 {
            let mut val = 0;
            if let Some(node) = l1 {
                val += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                val += node.val;
                l2 = node.next;
            }
            val += rest;
            rest = val / 10;
            foo = Solution::push_val(foo, val % 10);
        }

        return result.next;
    }
}
/// @lc code=end
struct End;
