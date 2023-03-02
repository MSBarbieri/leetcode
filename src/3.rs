/// Category: algorithms
/// Level: Medium
/// Percent: 33.814285%

/// Given a string s, find the length of the longest substring without repeating characters.
///
///
/// Example 1:
///
/// Input: s = "abcabcbb"
/// Output: 3
/// Explanation: The answer is "abc", with the length of 3.
///
///
/// Example 2:
///
/// Input: s = "bbbbb"
/// Output: 1
/// Explanation: The answer is "b", with the length of 1.
///
///
/// Example 3:
///
/// Input: s = "pwwkew"
/// Output: 3
/// Explanation: The answer is "wke", with the length of 3.
/// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
///
///
///
/// Constraints:
///
///
/// 	0 <= s.length <= 5 * 10⁴
/// 	s consists of English letters, digits, symbols and spaces.
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();
        if len <= 1 {
            return len as i32;
        }

        let mut result = 0;
        let mut start_i = 0;
        for i in 0..len {
            let c = s.chars().nth(i).unwrap();
            if let Some(si) = &s[start_i..i].find(|curr| curr == c) {
                // println!("works {c} in {si} on {start_i}..{i},{:?}", &s[start_i..i]);
                result = result.max(i - start_i);
                start_i = start_i + si + 1;
            }
        }
        if len != start_i {
            result = result.max(len - start_i);
        }

        result as i32
    }
}
/// @lc code=end
struct End;
