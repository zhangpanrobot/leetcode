/*
 * @lc app=leetcode id=389 lang=rust
 *
 * [389] Find the Difference
 */

pub struct Solution {} 

// @lc code=start
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        // ASCII code
        // let sum_s: u8 = s.chars().map(|c| c as u8).sum();
        // let sum_t: u8 = t.chars().map(|c| c as u8).sum();
        // char::from(sum_t - sum_s)
        // XOR
        let mut ret = 0;
        for ch in s.bytes() {
            ret ^= ch
        }
        for ch in t.bytes() {
            ret ^= ch
        }
        ret as char
    }
}
// @lc code=end

