/// Category: algorithms
/// Level: Medium
/// Percent: 56.152855%

/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
///
/// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
///
///
/// Example 1:
///
/// Input: digits = "23"
/// Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
///
///
/// Example 2:
///
/// Input: digits = ""
/// Output: []
///
///
/// Example 3:
///
/// Input: digits = "2"
/// Output: ["a","b","c"]
///
///
///
/// Constraints:
///
///
/// 	0 <= digits.length <= 4
/// 	digits[i] is a digit in the range ['2', '9'].
///

pub struct Solution;
/// @lc code=start
impl Solution {
    pub fn get_letters(c: char) -> Vec<char> {
        return match c {
            '2' => vec!['a', 'b', 'c'],
            '3' => vec!['d', 'e', 'f'],
            '4' => vec!['g', 'h', 'i'],
            '5' => vec!['j', 'k', 'l'],
            '6' => vec!['m', 'n', 'o'],
            '7' => vec!['p', 'q', 'r', 's'],
            '8' => vec!['t', 'u', 'v'],
            '9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![],
        };
    }
    pub fn multiply_result(result: &mut Vec<String>, chars: Vec<char>) -> Vec<String> {
        let mut new_result = vec![];
        if result.len() > 0 {
            for c in chars.iter() {
                for r in result.iter() {
                    let mut text = r.clone();
                    text.push(*c);
                    // println!("{text}");
                    new_result.push(text);
                }
            }
        } else {
            for c in chars.iter() {
                let mut text = String::default();
                text.push(*c);
                new_result.push(text);
            }
        }

        return new_result;
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = vec![];
        for d in digits.chars() {
            let chars = Solution::get_letters(d);
            result = Solution::multiply_result(&mut result, chars)
        }

        return result;
    }
}
/// @lc code=end
struct End;
