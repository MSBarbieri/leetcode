/// Category: algorithms
/// Level: Medium
/// Percent: 46.08165%

/// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
///
///
/// Example 1:
///
/// Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
/// Output: [[1,6],[8,10],[15,18]]
/// Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
///
///
/// Example 2:
///
/// Input: intervals = [[1,4],[4,5]]
/// Output: [[1,5]]
/// Explanation: Intervals [1,4] and [4,5] are considered overlapping.
///
///
///
/// Constraints:
///
///
/// 	1 <= intervals.length <= 10⁴
/// 	intervals[i].length == 2
/// 	0 <= starti <= endi <= 10⁴
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a.get(0).unwrap().cmp(b.get(0).unwrap()));
        let mut result: Vec<Vec<i32>> = vec![];

        for x in intervals.iter() {
            if let Some(r) = result.last_mut() {
                let r_min = r.get(0).unwrap().clone();
                let r_max = r.get(1).unwrap().clone();
                let x_min = x.get(0).unwrap().clone();
                let x_max = x.get(1).unwrap().clone();

                let min = r_min.min(x_min);
                let max = r_max.max(x_max);

                if r_max >= x_min && r_min <= x_max {
                    r.clear();
                    r.push(min.clone());
                    r.push(max.clone());
                } else {
                    result.push(x.clone());
                }
            } else {
                result.push(x.clone());
            }
        }

        return result;
    }
}
/// @lc code=end
struct End;
