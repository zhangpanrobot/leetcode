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
      let j = 0;
      let mut s:i32 = target;
      while j < len {
        s = s - nums[i];
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

mod tests {
  use super::*;

  #[test]
  fn test_209() {
      assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
      assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
  }
}