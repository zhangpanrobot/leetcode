/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */
pub struct Solution {}

// @lc code=start
// https://leetcode.com/problems/move-zeroes/discuss/172432/THE-EASIEST-but-UNUSUAL-snowball-JAVA-solution-BEATS-100-(O(n))-%2B-clear-explanation
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) -> &mut Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let mut snow_ball_size = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                snow_ball_size += 1;
            } else if snow_ball_size > 0 {
                let t = nums[i];
                nums[i] = 0;
                nums[i - snow_ball_size] = t;
            }
        }
        return nums;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let mut vec = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut vec);
        assert_eq!(vec, vec![1, 3, 12, 0, 0]);
    }
}
