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
pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum = 0;
    _distribute_coins(root, &mut sum);
    sum
}

pub fn _distribute_coins(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> (i32, i32) {
    if let Some(node) = root {
        let (left_size, left_coins) = _distribute_coins(node.borrow().left.clone(), sum);
        let (right_size, right_coins) = _distribute_coins(node.borrow().right.clone(), sum);
        let cur_size = left_size + right_size + 1;
        let cur_coins = left_coins + right_coins + node.borrow().val;
        *sum += (cur_coins - cur_size).abs();
        return (cur_size, cur_coins);
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
