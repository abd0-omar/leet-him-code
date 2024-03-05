//TODO: do it using the tuple return

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
pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    _max_path_sum(root)
}

pub fn _max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let mut max = node.val;
        let l_max_path = depth_sum(node.left.clone());
        println!("l_max_path={:?}", l_max_path);
        let r_max_path = depth_sum(node.right.clone());
        println!("r_max_path={:?}", r_max_path);

        max = std::cmp::max(
            std::cmp::max(max, l_max_path + r_max_path + node.val),
            std::cmp::max(l_max_path + node.val, r_max_path + node.val),
        )
        .max(std::cmp::max(
            _max_path_sum(node.left.clone()),
            _max_path_sum(node.right.clone()),
        ));

        return max;
    }
    -1000
}

// max sum for one branch
fn depth_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let node = node.borrow();
        let mut max = node.val;
        println!("max={:?}", max);
        let l = depth_sum(node.left.clone());
        println!("l={:?}", l);
        let r = depth_sum(node.right.clone());
        println!("r={:?}", r);

        max = (node.val + l.max(r)).max(max);

        return max;
    }
    0
}
