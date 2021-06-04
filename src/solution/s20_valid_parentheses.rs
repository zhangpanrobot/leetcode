/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */
 use std::collections::HashMap;
 pub struct Solution {}


// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let v: Vec<char> = Vec::new();
        let mut map:HashMap<char, char> = HashMap::new();
        let backet_v_left = vec!['(', '[', '{'];
        let backet_v_right = vec![')', ']', '}'];
        map.insert(')', '(');
        map.insert(']', '[');
        map.insert('}', '{');
        let res = true;
        for (i, c) in s.chars().enumerate() { 
            // do something with `c`
            if backet_v_left.contains(&c) {
                v.push(c);
            } else if map.contains_key(&c) && map.get(&c) == v[i - 1] {
                v.pop();
            } else {
                res = false;
                break;
            }
        }
        if v.is_empty() {
            res = true
        }
        return res;
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