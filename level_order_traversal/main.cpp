#include <algorithm>
#include <iostream>
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

int depth(TreeNode *root) {
  if (!root) {
    return 0;
  }

  int l = depth(root->left);
  int r = depth(root->right);

  return max(l, r) + 1;
}

void helperLevel(TreeNode *root, vector<vector<int>> &v, int level = 0) {
  if (!root) {
    return;
  }

  if (level == v.size()) {
    v.push_back(vector<int>{});
  }

  v[level].push_back(root->val);

  helperLevel(root->left, v, level + 1);
  helperLevel(root->right, v, level + 1);
}

vector<vector<int>> levelOrder(TreeNode *root) {
  vector<vector<int>> v{};
  helperLevel(root, v, 0);
  return v;
}

TreeNode *createTree1() {
  TreeNode *root = new TreeNode(1);
  root->left = new TreeNode(2);
  root->right = new TreeNode(3);
  root->left->left = new TreeNode(4);
  root->left->right = new TreeNode(5);
  root->right->left = new TreeNode(6);
  root->right->right = new TreeNode(7);
  return root;
}

TreeNode *createTree2() {
  TreeNode *root = new TreeNode(5);
  root->left = new TreeNode(2);
  root->right = new TreeNode(8);
  root->left->left = new TreeNode(1);
  root->left->right = new TreeNode(4);
  root->right->left = new TreeNode(6);
  root->right->right = new TreeNode(9);
  root->left->right->left = new TreeNode(3);
  root->right->left->right = new TreeNode(7);
  return root;
}

int main() {
  // Test case 1
  TreeNode *tree1 = createTree1();
  vector<vector<int>> result1 = levelOrder(tree1);
  cout << "Test Case 1 Result: ";
  for (const vector<int> &level : result1) {
    for (int num : level) {
      cout << num << " ";
    }
  }
  cout << endl;

  // Test case 2
  TreeNode *tree2 = createTree2();
  vector<vector<int>> result2 = levelOrder(tree2);
  cout << "Test Case 2 Result: ";
  for (const vector<int> &level : result2) {
    for (int num : level) {
      cout << num << " ";
    }
  }
  cout << endl;

  return 0;
}
