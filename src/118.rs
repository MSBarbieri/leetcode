/// Category: algorithms
/// Level: Easy
/// Percent: 70.597755%

/// Given an integer numRows, return the first numRows of Pascal's triangle.
///
/// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
///
///
/// Example 1:
/// Input: numRows = 5
/// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
/// Example 2:
/// Input: numRows = 1
/// Output: [[1]]
///
///
/// Constraints:
///
///
/// 	1 <= numRows <= 30
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn tri_gen(row: &Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut prev = 1;
        result.push(prev);
        for i in 1..row.len() {
            let cur = row.get(i).unwrap().clone();
            let val = prev + cur;
            result.push(val);
            prev = cur;
        }
        result.push(1);
        return result;
    }
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {
            return vec![];
        }
        let mut result: Vec<Vec<i32>> = vec![vec![1]];
        let mut last = vec![1];
        for _ in 1..num_rows {
            last = Self::tri_gen(&last);
            result.push(last.clone());
        }
        return result;
    }
}
/// @lc code=end
struct End;

#[test]
fn new_test() {
    println!("{:?}", Solution::generate(8));
}
