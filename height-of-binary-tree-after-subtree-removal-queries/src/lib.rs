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
use std::cmp::max;
use std::collections::HashMap;
use std::rc::Rc;
// this simple solution gets tle
// pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
//     let mut result = Vec::new();
//     for q in queries {
//         // let f = max_height(root.clone(), q) - 1;
//         result.push(max_height(root.clone(), q) - 1);
//         // println!("{}", f);
//     }
//     result
// }
//
// fn max_height(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> i32 {
//     if let Some(node) = root {
//         let node = node.borrow();
//         if node.val == target {
//             return 0;
//         }
//         return max_height(node.left.clone(), target).max(max_height(node.right.clone(), target))
//             + 1;
//     }
//     0
// }

pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
    let mut max_height_after_removal = HashMap::new();
    let mut current_max_height = 0;

    traverse_left_to_right(
        root.clone(),
        0,
        &mut current_max_height,
        &mut max_height_after_removal,
    );

    current_max_height = 0;
    traverse_right_to_left(
        root.clone(),
        0,
        &mut current_max_height,
        &mut max_height_after_removal,
    );

    queries
        .iter()
        .map(|&q| *max_height_after_removal.get(&q).unwrap_or(&0))
        .collect()
}

fn traverse_left_to_right(
    node: Option<Rc<RefCell<TreeNode>>>,
    current_height: i32,
    current_max_height: &mut i32,
    max_height_after_removal: &mut HashMap<i32, i32>,
) {
    if let Some(n) = node {
        let n = n.borrow();

        max_height_after_removal.insert(n.val, *current_max_height);

        *current_max_height = max(*current_max_height, current_height);

        traverse_left_to_right(
            n.left.clone(),
            current_height + 1,
            current_max_height,
            max_height_after_removal,
        );
        traverse_left_to_right(
            n.right.clone(),
            current_height + 1,
            current_max_height,
            max_height_after_removal,
        );
    }
}

fn traverse_right_to_left(
    node: Option<Rc<RefCell<TreeNode>>>,
    current_height: i32,
    current_max_height: &mut i32,
    max_height_after_removal: &mut HashMap<i32, i32>,
) {
    if let Some(n) = node {
        let n = n.borrow();

        let current_removal_height = max_height_after_removal.get(&n.val).cloned().unwrap();
        max_height_after_removal.insert(n.val, max(current_removal_height, *current_max_height));

        *current_max_height = max(*current_max_height, current_height);

        traverse_right_to_left(
            n.right.clone(),
            current_height + 1,
            current_max_height,
            max_height_after_removal,
        );
        traverse_right_to_left(
            n.left.clone(),
            current_height + 1,
            current_max_height,
            max_height_after_removal,
        );
    }
}
