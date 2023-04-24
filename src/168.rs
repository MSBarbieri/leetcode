/// Category: algorithms
/// Level: Easy
/// Percent: 35.612785%
/// Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
///
/// For example:
///
/// A -> 1
/// B -> 2
/// C -> 3
/// ...
/// Z -> 26
/// AA -> 27
/// AB -> 28
/// ...
///
///
///
/// Example 1:
///
/// Input: columnNumber = 1
/// Output: "A"
///
///
/// Example 2:
///
/// Input: columnNumber = 28
/// Output: "AB"
///
///
/// Example 3:
///
/// Input: columnNumber = 701
/// Output: "ZY"
///
///
///
/// Constraints:
///
///
/// 	1 <= columnNumber <= 2³¹ - 1
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut text = String::default();
        while column_number > 0 {
            column_number -= 1;
            text.push((b'A' + (column_number % 26) as u8) as char);
            column_number /= 26;
        }

        return text.chars().rev().collect();
    }
}
/// @lc code=end
struct End;

#[test]
fn foo() {
    let result = Solution::convert_to_title(27);
    assert_eq!("AA", result);
    let result = Solution::convert_to_title(28);
    assert_eq!("AB", result);
    let result = Solution::convert_to_title(701);
    assert_eq!("ZY", result);
    let result = Solution::convert_to_title(702);
    assert_eq!("ZZ", result);
    let result = Solution::convert_to_title(703);
    assert_eq!("AAA", result);
}
