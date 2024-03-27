#include <iostream>

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
public:
  TreeNode *invertTree(TreeNode *root) {
    if (!root) {
      return nullptr;
    }

    TreeNode *temp = root->left;
    root->left = invertTree(root->right);
    root->right = invertTree(temp);

    // root->left = invertTree(root->right);
    // root->right = invertTree(root->left);
    //
    return root;
  }
};

void printInOrder(TreeNode *root) {
  if (root == nullptr) {
    return;
  }
  printInOrder(root->left);
  std::cout << root->val << " ";
  printInOrder(root->right);
}

int main() {
  // Test case 1: Inverting a tree with a single node
  TreeNode *root1 = new TreeNode(1);
  Solution solution1;
  std::cout << "Original Tree: ";
  printInOrder(root1);
  std::cout << std::endl;
  TreeNode *invertedRoot1 = solution1.invertTree(root1);
  std::cout << "Inverted Tree: ";
  printInOrder(invertedRoot1);
  std::cout << std::endl;

  // Test case 2: Inverting a tree with multiple nodes
  TreeNode *root2 = new TreeNode(4);
  root2->left = new TreeNode(2);
  root2->right = new TreeNode(7);
  root2->left->left = new TreeNode(1);
  root2->left->right = new TreeNode(3);
  root2->right->left = new TreeNode(6);
  root2->right->right = new TreeNode(9);
  Solution solution2;
  std::cout << "Original Tree: ";
  printInOrder(root2);
  std::cout << std::endl;
  TreeNode *invertedRoot2 = solution2.invertTree(root2);
  std::cout << "Inverted Tree: ";
  printInOrder(invertedRoot2);
  std::cout << std::endl;

  // Clean up memory
  // TODO: Implement a function to delete the tree nodes and call it here

  return 0;
}
