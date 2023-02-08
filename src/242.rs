/// Category: algorithms
/// Level: Easy
/// Percent: 62.93674%

/// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
///
///
/// Example 1:
/// Input: s = "anagram", t = "nagaram"
/// Output: true
/// Example 2:
/// Input: s = "rat", t = "car"
/// Output: false
///
///
/// Constraints:
///
///
/// 	1 <= s.length, t.length <= 5 * 10⁴
/// 	s and t consist of lowercase English letters.
///
///
///
/// Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

struct Solution;
/// @lc code=start
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut chars = s.chars().collect::<Vec<char>>();
        for c in t.chars() {
            // println!("{:?}", chars);
            for i in 0..chars.len() {
                if chars[i] == c {
                    chars.remove(i);
                    break;
                }
            }
        }
        chars.is_empty()
    }
}
/// @lc code=end
struct End;
