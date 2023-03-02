/// Category: algorithms
/// Level: Medium
/// Percent: 27.349707%

/// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2³¹, 2³¹ - 1], then return 0.
///
/// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
///
///
/// Example 1:
///
/// Input: x = 123
/// Output: 321
///
///
/// Example 2:
///
/// Input: x = -123
/// Output: -321
///
///
/// Example 3:
///
/// Input: x = 120
/// Output: 21
///
///
///
/// Constraints:
///
///
/// 	-2³¹ <= x <= 2³¹ - 1
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut value: i64 = 0;
        while x != 0 {
            let digit = x % 10;
            x /= 10;
            value *= 10;
            value += digit as i64;
        }
        if value > i32::MAX as i64 || value < i32::MIN as i64 {
            0
        } else {
            value as i32
        }
    }
}
/// @lc code=end
struct End;
