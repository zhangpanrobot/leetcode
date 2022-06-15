/*
 * @lc app=leetcode id=27 lang=rust
 *
 * [27] Remove Element
 */
pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let len = nums.len();
        let mut start = 0;
        let mut end = len;
        while end > start {
            if nums[start] == val {
                nums.swap(start, end - 1);
                end -= 1;
            } else {
                start += 1;
            } 
        }
        return end as i32;
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut vec = vec![0, 1, 0, 3, 12, 3];
        let mut vec1 = vec![1];
        let mut vec2 = vec![1, 2];
        let mut vec3 = vec![1, 1];
        let res1 = Solution::remove_element(&mut vec, 3);
        let res2 = Solution::remove_element(&mut vec, 0);
        let res3 = Solution::remove_element(&mut vec, 2);
        let res4 = Solution::remove_element(&mut vec1, 1);
        let res5 = Solution::remove_element(&mut vec2, 1);
        let res6 = Solution::remove_element(&mut vec3, 1);
        assert_eq!(res1, 4);
        assert_eq!(res2, 4);
        assert_eq!(res3, 6);
        assert_eq!(res4, 0);
        assert_eq!(res5, 1);
        assert_eq!(res6, 0);
    }
}

