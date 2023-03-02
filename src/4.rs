/// Category: algorithms
/// Level: Hard
/// Percent: 35.77979%

/// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
///
/// The overall run time complexity should be O(log (m+n)).
///
///
/// Example 1:
///
/// Input: nums1 = [1,3], nums2 = [2]
/// Output: 2.00000
/// Explanation: merged array = [1,2,3] and median is 2.
///
///
/// Example 2:
///
/// Input: nums1 = [1,2], nums2 = [3,4]
/// Output: 2.50000
/// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
///
///
///
/// Constraints:
///
///
/// 	nums1.length == m
/// 	nums2.length == n
/// 	0 <= m <= 1000
/// 	0 <= n <= 1000
/// 	1 <= m + n <= 2000
/// 	-10⁶ <= nums1[i], nums2[i] <= 10⁶
///

struct Solution;
/// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut concated = [nums1, nums2].concat();
        concated.sort();

        let len = concated.len();
        if len % 2 == 0 {
            let a = concated.get(len / 2).unwrap();
            let b = concated.get((len / 2) - 1).unwrap();
            return (a.clone() + b.clone()) as f64 / 2.0;
        } else {
            return *concated.get(len / 2).unwrap() as f64;
        }
    }
}
/// @lc code=end
struct End;
