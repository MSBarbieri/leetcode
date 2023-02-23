/// Category: algorithms
/// Level: Medium
/// Percent: 66.63475%

/// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
///
///
/// Example 1:
/// Input: strs = ["eat","tea","tan","ate","nat","bat"]
/// Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
/// Example 2:
/// Input: strs = [""]
/// Output: [[""]]
/// Example 3:
/// Input: strs = ["a"]
/// Output: [["a"]]
///
///
/// Constraints:
///
///
/// 	1 <= strs.length <= 10⁴
/// 	0 <= strs[i].length <= 100
/// 	strs[i] consists of lowercase English letters.
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn is_anagram(a: &String, b: &String) -> bool {
        let mut _a = a.chars().collect::<Vec<char>>();
        let mut _b = b.chars().collect::<Vec<char>>();
        _a.sort();
        _b.sort();

        return _a.eq(&_b);
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        'strs: for t in strs.iter() {
            for n in 0..result.len() {
                if Solution::is_anagram(t, &result[n][0]) {
                    result[n].push(t.clone());
                    continue 'strs;
                }
            }
            result.push(vec![t.clone()]);
        }

        return result;
    }
}
/// @lc code=end
struct End;
