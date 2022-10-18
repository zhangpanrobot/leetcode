/*
 * @lc app=leetcode id=110 lang=cpp
 *
 * [110] Balanced Binary Tree
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
#include <algorithm>
#include <cstdlib>

// class Solution {
//  public:
//   bool isBalanced(TreeNode* root) {
//     if (root == nullptr) return true;
//     auto left = root->left;
//     auto right = root->right;
//     int leftDep = maxDepth(root->left);
//     int rightDep = maxDepth(root->right);
//     return std::abs(leftDep - rightDep) <= 1 && isBalanced(left) &&
//            isBalanced(right);
//   }

//   int maxDepth(TreeNode* root) {
//     if (root == nullptr) return 0;
//     return 1 + std::max(maxDepth(root->left), maxDepth(root->right));
//   }
// };

class Solution {
 public:
  bool isBalanced(TreeNode* root) { return dfsHeight(root) != -1; }

  int dfsHeight(TreeNode* root) {
    if (root == nullptr) return 0;
    int leftH = dfsHeight(root->left);
    if (leftH == -1) return -1;
    int rightH = dfsHeight(root->right);
    if (rightH == -1) return -1;

    if (std::abs(leftH - rightH) > 1) return -1;
    return 1 + std::max(leftH, rightH);
  }
};
// @lc code=end
