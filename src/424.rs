/// Category: algorithms
/// Level: Medium
/// Percent: 52.042244%

/// You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.
///
/// Return the length of the longest substring containing the same letter you can get after performing the above operations.
///
///
/// Example 1:
///
/// Input: s = "ABAB", k = 2
/// Output: 4
/// Explanation: Replace the two 'A's with two 'B's or vice versa.
///
///
/// Example 2:
///
/// Input: s = "AABABBA", k = 1
/// Output: 4
/// Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
/// The substring "BBBB" has the longest repeating letters, which is 4.
/// There may exists other ways to achive this answer too.
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 10⁵
/// 	s consists of only uppercase English letters.
/// 	0 <= k <= s.length
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn check(s: &[char], c_pos: usize, mut k: i32) -> usize {
        let mut size = 0;
        let mut _k = k;
        for i in 0..s.len() {
            if s[i] == s[c_pos] {
                size += 1;
            } else {
                if _k > 0 {
                    _k -= 1;
                    size += 1;
                    continue;
                }
                break;
            }
        }
        let mut new_size = 0;
        for i in c_pos..s.len() {
            if s[i] == s[c_pos] {
                new_size += 1;
            } else {
                if _k > 0 {
                    _k -= 1;
                    new_size += 1;
                    continue;
                }
                break;
            }
        }

        return usize::max(size, new_size);
    }

    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut prev_char = 0;
        let mut max_len = 0;
        while i < s.len() - 1 {
            let z = Self::check(&s[prev_char..s.len()], i, k);
            max_len = i32::max(z as i32, max_len);

            i += 1;
            let mut j = i;
            let mut _k = k;
            while j > 0 {
                if s[j] != s[i] {
                    if _k > 0 {
                        _k -= 1;
                    } else {
                        break;
                    }
                }
                j -= 1;
            }
            prev_char = j;
        }

        return max_len;
    }
}
/// @lc code=end
struct End;
