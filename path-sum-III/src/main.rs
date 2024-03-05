fn main() {
    // Construct a sample binary tree
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -2,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    let target_sum = -1;

    let _x = path_sum(root, target_sum);

    // let prefix_sum = get_prefix_sum_vec(root);
    // println!("Prefix Sum: {:?}", prefix_sum);
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
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

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    // hm.add(0, 1)
    let mut hm = std::collections::HashMap::new();
    hm.insert(0, 1);

    let mut count = 0;
    dfs(root, target_sum, &mut hm, 0, &mut count);
    count
}

fn dfs(
    root: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    hm: &mut std::collections::HashMap<i32, i32>,
    mut cur_sum: i32,
    count: &mut i32,
) {
    if let Some(node) = root {
        let node = node.borrow();
        cur_sum += node.val;
        let diff = cur_sum - target_sum;
        // check if it's in the hm
        // match hm.get(&diff) {
        //     Some(_) => *count+=1,
        //     None => ,
        // }
        if hm.get(&diff).is_some() {
            *count += 1;
        }
        // that's down
        *hm.entry(cur_sum).or_insert(0) += 1;
        //
        dfs(node.left.clone(), target_sum, hm, cur_sum, count);
        dfs(node.right.clone(), target_sum, hm, cur_sum, count);
        //-=1 from the map
        // if let Some(key) = hm.get(&diff) {
        // if let Some((&key, &val)) = hm.get_key_value(&diff) {
        //     let v = val - 1;
        //     hm.remove(&diff);
        //     if val != 0 {
        //         hm.insert(key, v - 1);
        //     }
        // }
        if let Some(val) = hm.get_mut(&cur_sum) {
            *val -= 1;
            if *val == 0 {
                hm.remove(&cur_sum);
            }
        }
    }
}
