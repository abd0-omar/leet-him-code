#include <iostream>

struct ListNode {
  int val;
  ListNode *next;
  ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
  void deleteNode(ListNode *node) {
    ListNode *curr = node;
    (*curr).val = (*(*curr).next).val;
    curr->next = curr->next->next;
  }
};

int main() { return 0; }
