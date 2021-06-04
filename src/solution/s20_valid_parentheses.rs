/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */
pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let bracket1_left = '(';
        let bracket2_left = '[';
        let bracket3_left = '{';
        let bracket1_right = ')';
        let bracket2_right = ']';
        let bracket3_right = '}';
        return false;
    }
}
// @lc code=end

mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let case1 = "[[[]]]";
        let case2 = "[([]]]";
        let case3 = "[[[]]}";
        assert!(Solution::is_valid(case1.to_owned()));
        assert!(Solution::is_valid(case2.to_owned()));
        assert!(Solution::is_valid(case3.to_owned()));
    }
}