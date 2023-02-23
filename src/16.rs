/// Category: algorithms
/// Level: Medium
/// Percent: 45.924366%

/// Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.
///
/// Return the sum of the three integers.
///
/// You may assume that each input would have exactly one solution.
///
///
/// Example 1:
///
/// Input: nums = [-1,2,1,-4], target = 1
/// Output: 2
/// Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
///
///
/// Example 2:
///
/// Input: nums = [0,0,0], target = 1
/// Output: 0
/// Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
///
///
///
/// Constraints:
///
///
/// 	3 <= nums.length <= 500
/// 	-1000 <= nums[i] <= 1000
/// 	-10⁴ <= target <= 10⁴
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut val = i32::MAX;
        if nums.len() < 3 {
            return 0;
        }

        for i in 0..nums.len() - 3 {
            let num_i = nums[i];
            for j in i..nums.len() - 2 {
                let num_j = nums[j];
                for k in j..nums.len() - 1 {
                    let num_k = nums[k];
                    val = val.min(num_i + num_j + num_k);
                    if val == target {
                        return val;
                    }
                }
            }
        }

        val
    }
}
/// @lc code=end
struct End;
