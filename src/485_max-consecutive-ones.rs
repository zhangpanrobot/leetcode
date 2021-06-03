/*
 * @lc app=leetcode id=485 lang=rust
 *
 * [485] Max Consecutive Ones
 */

// @lc code=start
use std::cmp;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut res = 0;
        for elem in nums {
            if elem == 1 {
                count += 1;
                res = cmp::max(count, res);
            } else {
                count = 0;
            }
        }
        return res;
    }
}
// @lc code=end

