/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
 */

pub struct Solution {}
use crate::util::linked_list::{to_list,  ListNode};

// @lc code=start
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut next = dummy.as_mut();
        dummy.unwrap().next
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_206() {
        assert_eq!(
            Solution::reverse_list(linked![1, 2, 4, 3, 5]),
            linked![5, 3, 4, 2, 1]
        );
    } 
}
