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
pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut hm = std::collections::HashMap::new();
    _find_frequent_tree_sum(root, &mut hm);
    println!("vec={:?}", hm);

    let max_val = hm.values().max().unwrap_or(&0);
    let mut rez = vec![];
    for (key, val) in hm.iter() {
        if val == max_val {
            rez.push(*key);
        }
    }

    rez
}

pub fn _find_frequent_tree_sum(
    root: Option<Rc<RefCell<TreeNode>>>,
    hm: &mut std::collections::HashMap<i32, i32>,
) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let mut sum_subtree = node.val;

        // if let Some(l) = node.left.clone() {
        //     sum_subtree += l.borrow().val;
        // }
        //
        // if let Some(r) = node.right.clone() {
        //     sum_subtree += r.borrow().val;
        // }

        sum_subtree += _find_frequent_tree_sum(node.left.clone(), hm);
        sum_subtree += _find_frequent_tree_sum(node.right.clone(), hm);

        println!("sum_subtree={:?}", sum_subtree);

        *hm.entry(sum_subtree).or_insert(0) += 1;

        return sum_subtree;
    }
    0
}
