/*
 * @lc app=leetcode id=101 lang=cpp
 *
 * [101] Symmetric Tree
 */

// @lc code=start
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left),
 * right(right) {}
 * };
 */
class Solution {
 public:
  bool isSymmetric(TreeNode *root) {
    return root == nullptr || isSymmetricHelp(root->left, root->right);
  }

  bool isSymmetricHelp(TreeNode *left, TreeNode *right) {
    bool leftNull = left == nullptr;
    bool rightNull = right == nullptr;
    if (leftNull || rightNull) return !(leftNull ^ rightNull);
    if (left->val != right->val) return false;
    return isSymmetricHelp(left->left, right->right) &&
           isSymmetricHelp(left->right, right->left);
  }
};
// @lc code=end
