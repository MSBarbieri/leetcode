/// Category: algorithms
/// Level: Medium
/// Percent: 64.13159%

/// Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
///
///
/// Example 1:
/// Input: nums = [1,1,1,2,2,3], k = 2
/// Output: [1,2]
/// Example 2:
/// Input: nums = [1], k = 1
/// Output: [1]
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁵
/// 	-10⁴ <= nums[i] <= 10⁴
/// 	k is in the range [1, the number of unique elements in the array].
/// 	It is guaranteed that the answer is unique.
///
///
///
/// Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.

struct Solution;
/// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in nums.iter() {
            map.entry(i).and_modify(|v| *v += 1).or_insert(1);
        }
        let mut heap = std::collections::BinaryHeap::new();
        for (k, v) in map.into_iter() {
            heap.push((v, k));
        }
        let mut i = 0;
        let mut res = vec![0; k as usize];
        while i < k as usize {
            res[i] = *heap.pop().unwrap().1;
            i += 1;
        }
        res
    }
}
/// @lc code=end
struct End;
