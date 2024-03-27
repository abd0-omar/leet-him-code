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

  int count(TreeNode *root) {
    if (!root) {
      return 0;
    }

    int l = count(root->left);
    int r = count(root->right);

    return l + r + 1;
  }

public:
  bool isSameTree(TreeNode *p, TreeNode *q) {
    if (!p && !q) {
      return true;
    }

    if (!p && q) {
      return false;
    }

    if (p && !q) {
      return false;
    }

    if (p == q) {
      return true;
    }

    return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
  }
};
