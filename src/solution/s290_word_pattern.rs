/*
 * @lc app=leetcode id=290 lang=rust
 *
 * [290] Word Pattern
 */
pub struct Solution {}

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut letterMap:HashMap<char, &str> = HashMap::new();
        let mut pIndex = 0;
        let mut presIndex = 0;
        let pLen = pattern.len();
        let patternList:char = pattern.chars().collect();
        let sLen = s.len();
        for (sIndex, c) in s.chars().enumerate() {
            if c == ' ' && pIndex < pLen {
                let letter = &s[presIndex..sIndex];
                let cLetter = letterMap.entry(patternList[pIndex]).or_insert(letter);
                if letter != cLetter {
                    break;
                }
                pIndex += 1;
            }
        }
        if pIndex < pLen {
            false
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_283() {
        let res 1 = Solution::word_pattern('abab', '');
        assert_eq!(vec, vec![1, 3, 12, 0, 0]);
    }
}

