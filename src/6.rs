/// Category: algorithms
/// Level: Medium
/// Percent: 43.533524%

/// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
///
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
///
///
/// And then read line by line: "PAHNAPLSIIGYIR"
///
/// Write the code that will take a string and make this conversion given a number of rows:
///
/// string convert(string s, int numRows);
///
///
///
/// Example 1:
///
/// Input: s = "PAYPALISHIRING", numRows = 3
/// Output: "PAHNAPLSIIGYIR"
///
///
/// Example 2:
///
/// Input: s = "PAYPALISHIRING", numRows = 4
/// Output: "PINALSIGYAHRPI"
/// Explanation:
/// P     I    N
/// A   L S  I G
/// Y A   H R
/// P     I
///
///
/// Example 3:
///
/// Input: s = "A", numRows = 1
/// Output: "A"
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 1000
/// 	s consists of English letters (lower-case and upper-case), ',' and '.'.
/// 	1 <= numRows <= 1000
///

struct Solution;
/// @lc code=start

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if s.len() <= 1 || num_rows == 1 {
            return s;
        }
        let mut texts: Vec<String> = (0..num_rows).map(|_| String::default()).collect();
        let mut i = 0;
        let mut cur_row = 0;
        let mut dir = 1;
        let len = s.len();
        while i < len {
            // println!("{i} {cur_row} {s}");
            let row = texts.get_mut(cur_row).unwrap();
            row.push(s.chars().nth(i).unwrap());

            if dir == 1 {
                cur_row += 1;
                if cur_row == (num_rows - 1) as usize {
                    dir = -1;
                }
            } else {
                cur_row -= 1;
                if cur_row == 0 {
                    dir = 1;
                }
            }
            i += 1;
        }
        texts.iter().flat_map(|s| s.chars()).collect()
    }
}
/// @lc code=end
struct End;
