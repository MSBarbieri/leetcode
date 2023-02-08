/// Category: algorithms
/// Level: Easy
/// Percent: 63.94148%

/// Given an array nums of size n, return the majority element.
///
/// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
///
///
/// Example 1:
/// Input: nums = [3,2,3]
/// Output: 3
/// Example 2:
/// Input: nums = [2,2,1,1,1,2,2]
/// Output: 2
///
///
/// Constraints:
///
///
/// 	n == nums.length
/// 	1 <= n <= 5 * 10⁴
/// 	-10⁹ <= nums[i] <= 10⁹
///
///
///
/// Follow-up: Could you solve the problem in linear time and in O(1) space?

struct Solution;
/// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut result = (1, *nums.first().unwrap());
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums.iter() {
            if map.contains_key(n) {
                let value = map.get(n).unwrap();
                if *value + 1 > result.0 {
                    result = (*value, *n);
                }
                map.insert(*n, *value + 1);
            } else {
                map.insert(*n, 1);
            }
        }

        return result.1;
    }
}
/// @lc code=end
struct End;
