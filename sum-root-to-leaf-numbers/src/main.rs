fn main() {}
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
pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut all_numbers = Vec::new();
    let x = _sum_numbers(root, String::new(), &mut all_numbers);
    // println!("{x}");
    // println!("all_numbers: {:?}", all_numbers);
    let new_vec = all_numbers
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    new_vec.iter().sum()
}

pub fn _sum_numbers(
    root: Option<Rc<RefCell<TreeNode>>>,
    mut string_no: String,
    all_numbers: &mut Vec<String>,
) -> () {
    if let Some(node) = root {
        // let mut string_no = node.borrow().val.to_string();
        string_no.push_str(&node.borrow().val.to_string());
        /*if let Some(n) = node.borrow().left.clone() {
            // string_no.push_str(&_sum_numbers(node.borrow().left.clone(), all_numbers));
            string_no.push_str(&n.borrow().val.to_string());
        }
        if let Some(n) = node.borrow().right.clone() {
            // string_no.push_str(&_sum_numbers(node.borrow().right.clone(), all_numbers));
            string_no.push_str(&n.borrow().val.to_string());
        }
        */
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            all_numbers.push(string_no.clone());
            // string_no.clear();
        }
        _sum_numbers(node.borrow().left.clone(), string_no.clone(), all_numbers);
        _sum_numbers(node.borrow().right.clone(), string_no, all_numbers);
        // return string_no;
    }
    // String::new()
}
