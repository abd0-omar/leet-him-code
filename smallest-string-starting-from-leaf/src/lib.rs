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
pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut smallest_string = None;
    _smallest_from_leaf(root, String::new(), &mut smallest_string);
    smallest_string
        .unwrap_or(String::new())
        .chars()
        .rev()
        .collect()
}

pub fn _smallest_from_leaf(
    root: Option<Rc<RefCell<TreeNode>>>,
    mut cur_string: String,
    smallest_string: &mut Option<String>,
) {
    if let Some(node) = root {
        let node_string = node.borrow().val;
        let node_string = (node_string + 97) as u8 as char;

        cur_string.push(node_string);
        if node.borrow().left.is_none() && node.borrow().right.is_none() {
            if let Some(smallest) = smallest_string {
                let smallest_reversed: String = smallest.chars().rev().collect();
                let cur_string_reversed: String = cur_string.chars().rev().collect();
                if cur_string_reversed < smallest_reversed {
                    *smallest = cur_string.clone();
                }
            } else {
                *smallest_string = Some(cur_string.clone());
            }
        }

        _smallest_from_leaf(
            node.borrow().left.clone(),
            cur_string.clone(),
            smallest_string,
        );
        _smallest_from_leaf(node.borrow().right.clone(), cur_string, smallest_string);
    }
}
