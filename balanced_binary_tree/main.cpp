#include <algorithm>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

class Solution {

  int height(TreeNode *root) {
    if (!root) {
      return 0;
    }

    int l = height(root->left);
    int r = height(root->right);

    return std::max(l, r) + 1;
  }

public:
  bool isBalanced(TreeNode *root) {
    if (!root) {
      return true;
    }

    int l = height(root->left);
    int r = height(root->right);

    if (l - r >= 2 || r - l >= 2) {
      return false;
    }

    return isBalanced(root->left) && isBalanced(root->right);
  }
};

#include <iostream>

// Define TreeNode struct and Solution class here

int main() {
  Solution solution;

  // Test case 1: Balanced tree with a single node
  TreeNode *root1 = new TreeNode(1);
  bool isBalanced1 = solution.isBalanced(root1);
  std::cout << "Test case 1: " << (isBalanced1 ? "Balanced" : "Not Balanced")
            << std::endl;

  // Test case 2: Balanced tree with multiple nodes
  TreeNode *root2 = new TreeNode(1);
  root2->left = new TreeNode(2);
  root2->right = new TreeNode(3);
  root2->left->left = new TreeNode(4);
  root2->left->right = new TreeNode(5);
  root2->right->left = new TreeNode(6);
  root2->right->right = new TreeNode(7);
  bool isBalanced2 = solution.isBalanced(root2);
  std::cout << "Test case 2: " << (isBalanced2 ? "Balanced" : "Not Balanced")
            << std::endl;

  // Test case 3: Unbalanced tree with left subtree taller
  TreeNode *root3 = new TreeNode(1);
  root3->left = new TreeNode(2);
  root3->left->left = new TreeNode(3);
  bool isBalanced3 = solution.isBalanced(root3);
  std::cout << "Test case 3: " << (isBalanced3 ? "Balanced" : "Not Balanced")
            << std::endl;

  // Test case 4: Unbalanced tree with right subtree taller
  TreeNode *root4 = new TreeNode(1);
  root4->right = new TreeNode(2);
  root4->right->right = new TreeNode(3);
  bool isBalanced4 = solution.isBalanced(root4);
  std::cout << "Test case 4: " << (isBalanced4 ? "Balanced" : "Not Balanced")
            << std::endl;

  // Test case 5: Unbalanced tree with large height difference
  TreeNode *root5 = new TreeNode(1);
  root5->left = new TreeNode(2);
  root5->left->left = new TreeNode(3);
  root5->left->left->left = new TreeNode(4);
  bool isBalanced5 = solution.isBalanced(root5);
  std::cout << "Test case 5: " << (isBalanced5 ? "Balanced" : "Not Balanced")
            << std::endl;

  // Clean up memory
  // TODO: Implement a function to delete the tree nodes and call it here

  return 0;
}
