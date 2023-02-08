/// Category: algorithms
/// Level: Easy
/// Percent: 55.888905%

/// You have a water dispenser that can dispense cold, warm, and hot water. Every second, you can either fill up 2 cups with different types of water, or 1 cup of any type of water.
///
/// You are given a 0-indexed integer array amount of length 3 where amount[0], amount[1], and amount[2] denote the number of cold, warm, and hot water cups you need to fill respectively. Return the minimum number of seconds needed to fill up all the cups.
///
///
/// Example 1:
///
/// Input: amount = [1,4,2]
/// Output: 4
/// Explanation: One way to fill up the cups is:
/// Second 1: Fill up a cold cup and a warm cup.
/// Second 2: Fill up a warm cup and a hot cup.
/// Second 3: Fill up a warm cup and a hot cup.
/// Second 4: Fill up a warm cup.
/// It can be proven that 4 is the minimum number of seconds needed.
///
///
/// Example 2:
///
/// Input: amount = [5,4,4]
/// Output: 7
/// Explanation: One way to fill up the cups is:
/// Second 1: Fill up a cold cup, and a hot cup.
/// Second 2: Fill up a cold cup, and a warm cup.
/// Second 3: Fill up a cold cup, and a warm cup.
/// Second 4: Fill up a warm cup, and a hot cup.
/// Second 5: Fill up a cold cup, and a hot cup.
/// Second 6: Fill up a cold cup, and a warm cup.
/// Second 7: Fill up a hot cup.
///
///
/// Example 3:
///
/// Input: amount = [5,0,0]
/// Output: 5
/// Explanation: Every second, we fill up a cold cup.
///
///
///
/// Constraints:
///
///
/// 	amount.length == 3
/// 	0 <= amount[i] <= 100
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut rest: Vec<i32> = amount.into_iter().filter(|c| *c > 0).collect();
        if rest.len() == 0 {
            return 0;
        }
        // println!("filtered {:?}", rest);
        rest.sort_by(|a, b| b.cmp(a));
        while rest.len() > 1 {
            rest[0] -= 1;
            rest[1] -= 1;
            rest = rest.into_iter().filter(|c| *c > 0).collect();
            rest.sort_by(|a, b| b.cmp(a));
            result += 1;
            // println!("result = {result} iter {:?}", rest);
        }

        if let Some(value) = rest.first() {
            result += value;
        }

        result
    }
}
/// @lc code=end
struct End;
