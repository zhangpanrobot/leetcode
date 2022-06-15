/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

// @lc code=start
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Solution::is_match_slice(s.as_bytes(), p.as_bytes())
    }
    pub fn is_match_slice(s: &[u8], p: &[u8]) -> bool {
        match (p, s) {
            ([x, b'*', subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => Solution::is_match_slice(subs, p),
            ([_, b'*', subp @ ..], _) => Solution::is_match_slice(s, subp),
            ([x, subp @ ..], [y, subs @ ..]) if *x == b'.' || x == y => Solution::is_match_slice(subs, subp),
            ([], s) => s.is_empty(),
            _ => false,
        }
    }
}
// @lc code=end

