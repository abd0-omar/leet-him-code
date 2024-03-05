fn main() {
    println!("Hello, world!");
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
use std::cell::RefCell;
use std::rc::Rc;
pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        // let mut count = _longest_zig_zag(Some(node.clone()), true);
        // let node = node.borrow();
        //
        // count.max(longest_zig_zag(node.left.clone()).max(longest_zig_zag(node.right.clone())))

        unimplemented!()
    } else {
        0
    }
}

// count for root node only
// pub fn _longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>, right: bool) -> i32 {
//     if let Some(node) = root {
//         let node = node.borrow();
//
//         let mut count = 1;
//
//         if let Some(l) = node.left.clone() {
//             if right == false {
//                 count += _longest_zig_zag(Some(l), true);
//             }
//         }
//
//         if let Some(r) = node.right.clone() {
//             if right == true {
//                 count += _longest_zig_zag(Some(r), false);
//             }
//         }
//
//         count
//     } else {
//         0
//     }
// }
