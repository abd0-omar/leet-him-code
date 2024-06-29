// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        _construct_maximum_binary_tree(&nums, 0, nums.len() as i32 - 1)
    }
}

fn _construct_maximum_binary_tree(nums: &Vec<i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
    // while l <= r
    if l > r {
        return None;
    }

    let mut max = 0;
    let mut max_index = -1;
    for i in l..=r {
        let i = i as usize;
        if nums[i] >= max {
            max = nums[i];
            max_index = i as i32;
        }
    }
    //println!("max: {}", max);
    //println!("l: {}", l);
    //println!("r: {}", r);
    // 1, 3, 2
    // 0  1  2
    // 
    let mut tree = Rc::new(RefCell::new(TreeNode::new(max)));
    tree.borrow_mut().left = _construct_maximum_binary_tree(nums, l, max_index - 1);
    tree.borrow_mut().right = _construct_maximum_binary_tree(nums, max_index + 1, r);

    return Some(tree);
}
