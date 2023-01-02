/*
 * @lc app=leetcode id=92 lang=cpp
 *
 * [92] Reverse Linked List II
 */

struct ListNode {
  int val;
  ListNode* next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode* next) : val(x), next(next) {}
};

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
 public:
  ListNode* reverseBetween(ListNode* head, int left, int right) {
    if (head == nullptr) return head;
    int curIndex = 1;
    ListNode* reserveNode = head;
    ListNode* curNode = head;
    ListNode* prevNode = head;
    ListNode* leftNode;

    while (curIndex <= right && curNode != nullptr) {
      if (curIndex == left - 1) {
        reserveNode = prevNode->next;
        leftNode = curNode->next;
      }
      ListNode* temp = curNode->next;
      if (curIndex > left && temp != nullptr) {
        curNode->next = prevNode;
        prevNode = curNode;
        curNode = temp;
      }
      if (curIndex == right) {
        reserveNode->next = curNode;
        leftNode->next = curNode->next;
      }
      if (temp) {
        curIndex++;
      }
    }
    return head;
  }
};
// @lc code=end
