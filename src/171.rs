/// Category: algorithms
/// Level: Easy
/// Percent: 62.120186%

/// Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.
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
/// Input: columnTitle = "A"
/// Output: 1
///
///
/// Example 2:
///
/// Input: columnTitle = "AB"
/// Output: 28
///
///
/// Example 3:
///
/// Input: columnTitle = "ZY"
/// Output: 701
///
///
///
/// Constraints:
///
///
/// 	1 <= columnTitle.length <= 7
/// 	columnTitle consists only of uppercase English letters.
/// 	columnTitle is in the range ["A", "FXSHRXW"].
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title
            .chars()
            .fold(0, |acc, c| acc * 26 + (c as i32 - 'A' as i32 + 1))
    }
}
/// @lc code=end
struct End;
#[test]
fn ascii_to_number() {
    assert_eq!(Solution::title_to_number(String::from("AB")), 28);
    assert_eq!(Solution::title_to_number(String::from("ZY")), 701);
}
