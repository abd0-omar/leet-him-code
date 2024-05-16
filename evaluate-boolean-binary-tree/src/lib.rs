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

struct Solution;
impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let node = node.borrow();

            if node.left.is_none() && node.right.is_none() {
                if node.val == 0 {
                    return false;
                }
                return true;
            }

            // if let Some(n) = node.left.clone() {
            //     if n.borrow().left.is_none() && n.borrow().right.is_none() {
            //         if n.borrow().val == 0 {
            //             return false;
            //         }
            //         return true;
            //     }
            // }
            //
            // if let Some(n) = node.right.clone() {
            //     if n.borrow().left.is_none() && n.borrow().right.is_none() {
            //         if n.borrow().val == 0 {
            //             return false;
            //         }
            //         return true;
            //     }
            // }

            let left = Self::evaluate_tree(node.left.clone());
            let right = Self::evaluate_tree(node.right.clone());

            if node.val == 2 {
                println!("happened or");
                println!("left={:?}", left);
                println!("right={:?}", right);
                return left || right;
            }
            if node.val == 3 {
                println!("happened and");
                println!("left={:?}", left);
                println!("right={:?}", right);
                return left && right;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
