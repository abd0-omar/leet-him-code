fn main() {
    println!("Hello, world!");
}

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
pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    if let Some(node) = root {
        let node = node.borrow();

        let left = tree2str(node.left.clone());
        let right = tree2str(node.right.clone());

        if left.is_empty() && right.is_empty() {
            return node.val.to_string();
        } else if right.is_empty() {
            return format!("{}({})", node.val, left);
        } else {
            return format!("{}({})({})", node.val, left, right);
        }
    }

    String::new()
}

// pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//     if let Some(node) = root {
//         let node = node.borrow();
//         let mut res = String::from(node.val.to_string());
//
//         if let Some(l) = node.left.clone() {
//             res.push('(');
//             res.push_str(&tree2str(Some(l)));
//             res.push(')');
//         }
//
//         if let Some(r) = node.right.clone() {
//             res.push('(');
//             res.push_str(&tree2str(Some(r)));
//             res.push(')');
//         }
//
//         res.push(')');
//
//         res
//     } else {
//         String::new()
//     }
// }

// pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//     let mut res = String::new();
//     if let Some(node) = root {
//         let node = node.borrow();
//         res.push(char::from_digit(node.val as u32, 10).unwrap());
//         if let Some(l) = node.left.clone() {
//             res.push_str(&_tree2str(Some(l)));
//         }
//         if let Some(r) = node.right.clone() {
//             res.push_str(&_tree2str(Some(r)));
//         }
//     }
//     res
// }

// pub fn _tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
//     if let Some(node) = root {
//         let node = node.borrow();
//
//         let mut str = String::from("(");
//         str.push(char::from_digit(node.val as u32, 10).unwrap());
//
//         str.push_str(&_tree2str(node.left.clone()));
//
//         str.push_str(&_tree2str(node.right.clone()));
//
//         str.push(')');
//         str
//     } else {
//         String::new()
//     }
// }
