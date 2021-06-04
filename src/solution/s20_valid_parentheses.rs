/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */
 pub struct Solution {}
 
 // @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                ')'|'}'|']' if Some(c) != stack.pop() => return false,
                _ => (),
            }
        }
        return stack.is_empty();
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
        let case4 = "[[[[[]]]";
        let case5 = "(]";
        assert!(Solution::is_valid(case1.to_owned()));
        assert!(Solution::is_valid(case2.to_owned()));
        assert!(Solution::is_valid(case3.to_owned()));
        assert!(Solution::is_valid(case4.to_owned()));
        assert!(Solution::is_valid(case5.to_owned()));
    }
}