#include <algorithm>
#include <iostream>
#include <queue>
#include <vector>
using namespace std;

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
  vector<vector<int>> levelOrder(TreeNode *root) {
    if (!root) {
      return vector<vector<int>>();
    }
    queue<TreeNode *> nodes;
    vector<vector<int>> vbig;
    nodes.push(root);
    while (!nodes.empty()) {
      int sz = nodes.size();
      int sz_copy = sz;
      vector<int> vsmall;

      while (sz--) {
        TreeNode *curr = nodes.front();
        vsmall.push_back(curr->val);
        nodes.pop();
        if (curr->left) {
          nodes.push(curr->left);
        }
        if (curr->right) {
          nodes.push(curr->right);
        }
      }
      if (vsmall.size() % 2 == 0) {
        reverse(vsmall.begin(), vsmall.end());
      }
      vbig.push_back(vsmall);
    }
    return vbig;
  }
};
int main() {
  Solution solution;

  // Test case 1: Binary tree with single node
  TreeNode *root1 = new TreeNode(1);
  std::vector<std::vector<int>> result1 = solution.levelOrder(root1);
  std::cout << "Test case 1: ";
  for (const auto &level : result1) {
    for (int val : level) {
      std::cout << val << " ";
    }
    std::cout << "| ";
  }
  std::cout << std::endl;

  // Test case 2: Binary tree with multiple levels
  TreeNode *root2 = new TreeNode(3);
  root2->left = new TreeNode(9);
  root2->right = new TreeNode(20);
  root2->right->left = new TreeNode(15);
  root2->right->right = new TreeNode(7);
  std::vector<std::vector<int>> result2 = solution.levelOrder(root2);
  std::cout << "Test case 2: ";
  for (const auto &level : result2) {
    for (int val : level) {
      std::cout << val << " ";
    }
    std::cout << "| ";
  }
  std::cout << std::endl;

  // Test case 3: Binary tree with one level
  TreeNode *root3 = new TreeNode(2);
  root3->left = new TreeNode(4);
  root3->right = new TreeNode(6);
  std::vector<std::vector<int>> result3 = solution.levelOrder(root3);
  std::cout << "Test case 3: ";
  for (const auto &level : result3) {
    for (int val : level) {
      std::cout << val << " ";
    }
    std::cout << "| ";
  }
  std::cout << std::endl;

  // Clean up memory
  // TODO: Implement a function to delete the tree nodes and call it here

  return 0;
}
