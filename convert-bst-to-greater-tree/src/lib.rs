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
pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    _convert_bst(&root, &mut 0);
    root
}

pub fn _convert_bst(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> () {
    if let Some(node) = root {
        let mut node = node.borrow_mut();
        _convert_bst(&node.right, sum);
        *sum += node.val;
        node.val = *sum;
        _convert_bst(&node.left, sum);
    }
}

#[cfg(test)]
mod tests {}
