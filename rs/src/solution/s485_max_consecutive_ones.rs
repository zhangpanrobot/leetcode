/*
 * @lc app=leetcode id=485 lang=rust
 *
 * [485] Max Consecutive Ones
 */
pub struct Solution {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_485() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 0, 1, 1]), 2);
        assert_eq!(Solution::find_max_consecutive_ones(vec![1, 1, 1]), 3);
    }
}

