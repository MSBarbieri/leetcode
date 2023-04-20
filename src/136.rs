/// Category: algorithms
/// Level: Easy
/// Percent: 70.60654%

/// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
///
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
///
///
/// Example 1:
/// Input: nums = [2,2,1]
/// Output: 1
/// Example 2:
/// Input: nums = [4,1,2,1,2]
/// Output: 4
/// Example 3:
/// Input: nums = [1]
/// Output: 1
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 3 * 10⁴
/// 	-3 * 10⁴ <= nums[i] <= 3 * 10⁴
/// 	Each element in the array appears twice except for one element which appears only once.
///

struct Solution;

/// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut foo = HashMap::new();
        for n in nums {
            let possibility = foo.get_mut(&n);
            if let Some(v) = possibility {
                *v += 1;
            } else {
                foo.insert(n, 1);
            }
        }
        foo.into_iter()
            .filter(|(_, v)| *v == 1)
            .collect::<Vec<(i32, usize)>>()
            .first()
            .unwrap()
            .0
    }
}
/// @lc code=end
struct End;
