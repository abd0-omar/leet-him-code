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
//
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
pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();
    let mut v = Vec::new();
    traverse(root, &mut hm);
    // v.push(hm.ite
    let max = hm.values().max();
    if let Some(&_max) = max {
        for (&key, &val) in hm.iter() {
            if val == _max {
                v.push(key);
            }
        }
    }
    v
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, hm: &mut std::collections::HashMap<i32, i32>) {
    if let Some(node) = root {
        *hm.entry(node.borrow().val).or_insert(0) += 1;
        traverse(node.borrow().left.clone(), hm);
        traverse(node.borrow().right.clone(), hm);
    }
}

fn parenthesize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    if let Some(node) = root {
        let root_val = node.borrow().val.to_string();
        let mut repr = "(".to_string() + &root_val;
        // repr.push_str(&root_val);

        if let Some(l) = node.borrow().left.clone() {
            repr += &parenthesize(Some(l));
        } else {
            repr += "()";
        }

        if let Some(r) = node.borrow().right.clone() {
            repr += &parenthesize(Some(r));
        } else {
            repr += "()";
        }

        repr += ")";

        return repr;
    }

    "()".to_string()
}

fn _find_mode(root: Option<Rc<RefCell<TreeNode>>>, frequency: &mut Vec<i32>) {
    unimplemented!()
}
