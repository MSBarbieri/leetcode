/// Category: algorithms
/// Level: Easy
/// Percent: 44.081673%

/// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
///
/// Given a string s, return true if it is a palindrome, or false otherwise.
///
///
/// Example 1:
///
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.
///
///
/// Example 2:
///
/// Input: s = "race a car"
/// Output: false
/// Explanation: "raceacar" is not a palindrome.
///
///
/// Example 3:
///
/// Input: s = " "
/// Output: true
/// Explanation: s is an empty string "" after removing non-alphanumeric characters.
/// Since an empty string reads the same forward and backward, it is a palindrome.
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 2 * 10⁵
/// 	s consists only of printable ASCII characters.
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let text = s.chars();
        let mut back_text = s.chars();
        for c in text {
            if !c.is_alphabetic() && !c.is_numeric() {
                continue;
            }
            if c.is_numeric() || c.is_alphabetic() {
                let mut cc = back_text.nth_back(0).unwrap();
                while !cc.is_numeric() && !cc.is_alphabetic() {
                    if let Some(_c) = back_text.nth_back(0) {
                        cc = _c;
                    }
                }

                if c.to_ascii_lowercase() != cc.to_ascii_lowercase() {
                    return false;
                }
            }
        }

        true
    }
}
/// @lc code=end
struct End;
