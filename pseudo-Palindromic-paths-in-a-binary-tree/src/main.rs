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
use std::collections::HashMap;
use std::rc::Rc;
pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut vec = Vec::new();
    _pseudo_palindromic_paths(root, vec![], &mut vec);

    let mut counter = 0;

    for v in &mut vec {
        v.sort_unstable();
    }

    println!("vec={:?}", vec);

    for v in &vec {
        let mut hm = HashMap::new();
        for i in 0..v.len() {
            *hm.entry(v[i]).or_insert(0) += 1;
        }
        let mut count = 0;
        for val in hm.values() {
            if val % 2 == 1 {
                count += 1;
            }
            if count == 2 {
                break;
            }
        }
        if count < 1 {
            counter += 1;
        }
    }

    counter
}

pub fn _pseudo_palindromic_paths(
    root: Option<Rc<RefCell<TreeNode>>>,
    mut v: Vec<i32>,
    vec: &mut Vec<Vec<i32>>,
) {
    if let Some(node) = root {
        let node = node.borrow();

        v.push(node.val);

        if node.left.is_none() && node.right.is_none() {
            vec.push(v.clone());
        }

        _pseudo_palindromic_paths(node.left.clone(), v.clone(), vec);
        _pseudo_palindromic_paths(node.right.clone(), v, vec);
    }
}
