/// Category: algorithms
/// Level: Medium
/// Percent: 40.034855%

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
pub struct Solution;
use crate::ListNode as RawListNode;

type ListNode = RawListNode<i32>;

/// @lc code=start
// Definition for singly-linked list.
impl ListNode {
    fn swap_node(cur: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match cur {
            Some(mut node_cur) => {
                let next = node_cur.next;
                node_cur.next = prev;
                return ListNode::swap_node(next, Some(node_cur));
            }
            None => (),
        }
        return prev;
    }

    fn reverse(start: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        ListNode::swap_node(start, None)
    }

    fn push(cur: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = cur {
            node.next = Some(Box::new(ListNode::new(val)));
            return node.next;
        }
        None
    }
}
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        l1 = ListNode::reverse(l1);
        l2 = ListNode::reverse(l2);
        println!("{:?}", l1);
        let result = Box::new(ListNode::new(0));
        let mut cur = Some(result.clone());
        let mut rest = 0;

        let live = true;
        while live {
            if l1.is_none() && l2.is_none() {
                break;
            }

            let l1_val = if let Some(node) = l1 {
                let v = node.val;
                l1 = node.next;
                v
            } else {
                0
            };

            let l2_val = if let Some(node) = l2 {
                let v = node.val;
                l2 = node.next;
                v
            } else {
                0
            };

            let val = rest + l1_val + l2_val;
            let sum = if val > 9 {
                rest = val / 10;
                val % 10
            } else {
                val
            };
            cur = ListNode::push(cur, sum);
        }

        return result.next;
    }
}
/// @lc code=end
struct End;
