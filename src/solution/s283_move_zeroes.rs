/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 */
pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        
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
