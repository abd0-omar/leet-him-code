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
use std::cell::RefCell;
use std::rc::Rc;
pub fn remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    _remove_leaf_nodes(root, target)
}

pub fn _remove_leaf_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let left = _remove_leaf_nodes(node.borrow_mut().left.take(), target);
        let right = _remove_leaf_nodes(node.borrow_mut().right.take(), target);
        node.borrow_mut().left = left;
        node.borrow_mut().right = right;

        if node.borrow().left.is_none()
            && node.borrow().right.is_none()
            && node.borrow().val == target
        {
            return None;
        }

        return Some(node);
    }
    None
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
