/// Category: algorithms
/// Level: Medium
/// Percent: 61.786507%

/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
///
/// For example, 2 is written as II in Roman numeral, just two one's added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:
///
///
/// 	I can be placed before V (5) and X (10) to make 4 and 9.
/// 	X can be placed before L (50) and C (100) to make 40 and 90.
/// 	C can be placed before D (500) and M (1000) to make 400 and 900.
///
///
/// Given an integer, convert it to a roman numeral.
///
///
/// Example 1:
///
/// Input: num = 3
/// Output: "III"
/// Explanation: 3 is represented as 3 ones.
///
///
/// Example 2:
///
/// Input: num = 58
/// Output: "LVIII"
/// Explanation: L = 50, V = 5, III = 3.
///
///
/// Example 3:
///
/// Input: num = 1994
/// Output: "MCMXCIV"
/// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
///
///
///
/// Constraints:
///
///
/// 	1 <= num <= 3999
///

struct Solution;
/// @lc code=start
pub fn push_text(num: i32, size: i32, text: &mut String, chars: &[char]) {
    let val = num / size;
    let last_number = val % 10;
    if last_number < 4 {
        for _ in 0..last_number {
            text.push(chars[0]);
        }
    } else if last_number <= 8 && last_number >= 4 {
        if last_number > 5 {
            text.push(chars[1]);
            for _ in 0..last_number - 5 {
                text.push(chars[0]);
            }
        } else if last_number < 5 {
            text.push(chars[0]);
            text.push(chars[1]);
        } else {
            text.push(chars[1]);
        }
    } else if last_number != 0 {
        text.push(chars[0]);
        text.push(chars[2]);
    }
}

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut text = String::default();
        if num >= 1000 {
            push_text(num, 1000, &mut text, &['M']);
            num = num % 1000;
        }
        if num < 1000 && num >= 100 {
            push_text(num, 100, &mut text, &['C', 'D', 'M']);
            num = num % 100;
        }
        if num < 100 && num >= 10 {
            push_text(num, 10, &mut text, &['X', 'L', 'C']);
            num = num % 10;
        }
        push_text(num, 1, &mut text, &['I', 'V', 'X']);

        return text;
    }
}
/// @lc code=end
struct End;
