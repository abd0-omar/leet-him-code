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
pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut sum = 0;
    _bst_to_gst(&root, &mut sum);
    root
}

pub fn _bst_to_gst(root: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> () {
    if let Some(node) = root {
        _bst_to_gst(&node.borrow().right.clone(), sum);
        println!("{}", node.borrow().val);
        *sum += node.borrow().val;
        node.borrow_mut().val = *sum;
        _bst_to_gst(&node.borrow().left.clone(), sum);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let root = [
        //     Some(4),
        //     Some(1),
        //     Some(6),
        //     Some(0),
        //     Some(2),
        //     Some(5),
        //     Some(7),
        //     None,
        //     None,
        //     None,
        //     Some(3),
        //     None,
        //     None,
        //     None,
        //     Some(8),
        // ];
        // let output = [
        //     Some(30),
        //     Some(36),
        //     Some(21),
        //     Some(36),
        //     Some(35),
        //     Some(26),
        //     Some(15),
        //     None,
        //     None,
        //     None,
        //     Some(33),
        //     None,
        //     None,
        //     None,
        //     Some(8),
        // ];

        // let result = bst_to_gst(root);

        // assert_eq!(result, output);
    }
}
