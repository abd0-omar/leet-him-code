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
  bool isSubtree(TreeNode *root, TreeNode *subRoot) {
    if (!root && !subRoot) {
      return true;
    }

    if (!root && subRoot) {
      return false;
    }

    if (root && !subRoot) {
      return false;
    }

    if (root != subRoot) {
      return false;
    }

    if (root->left && subRoot->left && root->left->val != subRoot->left->val) {
      return false;
    }

    if (root->right && subRoot->right &&
        root->right->val != subRoot->right->val) {
      return false;
    }

    return isSubtree(root->left, subRoot->left) &&
           isSubtree(root->right, subRoot->right);
  }
};
