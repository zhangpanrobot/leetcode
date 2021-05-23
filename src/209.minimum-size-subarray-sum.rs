/*
 * @lc app=leetcode id=209 lang=rust
 *
 * [209] Minimum Size Subarray Sum
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
      var len = len(nums)
      var res = len + 1
      var i = 0 
      for j = 0; j < len; j++ {
        s -= nums[i]
        while s <= 0 {
          res = min(res, j - i + 1)
          s += nums[i++]
        }
      }
      return res % (len + 1)
    }
}
// @lc code=end

