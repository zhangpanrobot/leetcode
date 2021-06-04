/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */
pub struct Solution {}

// @lc code=start
use std::cmp;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
      let len = nums.len();
      let mut res = len + 1;
      let mut i = 0;
      let mut j = 0;
      let mut s:i32;
      while j < len {
        s -= nums[i];
        while s <= 0 {
          res = cmp::min(res, j - i + 1);
          i += 1;
          s += nums[i];
        }
      }
      return (res % (len + 1)) as i32
    }
}
// @lc code=end

