// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if head.is_none() || root.is_none() {
            return false;
        }

        Solution::_is_sub_path(head.clone(), root.clone())
            || Solution::is_sub_path(head.clone(), root.clone().unwrap().borrow().left.clone())
            || Solution::is_sub_path(head.clone(), root.unwrap().borrow().right.clone())
    }

    pub fn _is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // dfs
        if head.is_none() {
            return true;
        }
        if root.is_none() {
            return false;
        }

        let head_node = head.as_ref().unwrap();
        let root_node = root.as_ref().unwrap().borrow();

        dbg!(&root_node.val);
        dbg!(&head_node.val);

        if head_node.val != root_node.val {
            return false;
        }

        Solution::_is_sub_path(head_node.next.clone(), root_node.left.clone())
            || Solution::_is_sub_path(head_node.next.clone(), root_node.right.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
