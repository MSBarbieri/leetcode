/// Category: algorithms
/// Level: Medium
/// Percent: 32.417385%

/// Given a string s, return the longest palindromic substring in s.
///
///
/// Example 1:
///
/// Input: s = "babad"
/// Output: "bab"
/// Explanation: "aba" is also a valid answer.
///
///
/// Example 2:
///
/// Input: s = "cbbd"
/// Output: "bb"
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 1000
/// 	s consist of only digits and English letters.
///

struct Solution;
/// @lc code=start
pub fn is_palindrome(s: &[u8]) -> bool {
    let mut lo = 0;
    let mut hi = s.len() - 1;
    while hi > lo {
        if s[lo] != s[hi] {
            return false;
        }
        lo += 1;
        hi -= 1;
    }
    return true;
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let b = s.as_bytes();
        let mut i = b.len() - 1;
        while i > 0 {
            for j in 0..b.len() - i {
                if is_palindrome(&b[j..=i + j]) {
                    return s[j..=i + j].to_string()
                }
            }

            i -= 1;
        }

        s.chars().nth(0).unwrap().to_string()
    }
}
/// @lc code=end
struct End;
