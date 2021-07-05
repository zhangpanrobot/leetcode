/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */
pub struct Solution {}


// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        nums.iter()
            .try_fold(HashSet::with_capacity(nums.len()), |mut s, x| {
                if s.insert(x) {
                    Some(s)
                } else {
                    None
                }
            }).is_none()
    }
}
// @lc code=end

