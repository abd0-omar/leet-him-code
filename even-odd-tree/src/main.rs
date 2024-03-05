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
pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    use std::collections::VecDeque;
    if let Some(node) = root {
        let mut q = VecDeque::new();
        q.push_back(node);

        let mut lvl = -1;
        while !q.is_empty() {
            println!("q={:?}", q);
            let size = q.len();
            lvl += 1;
            let mut prev = None;
            for _ in 0..size {
                println!("lvl={:?}", lvl);
                let popped = q.pop_front().unwrap();
                let popped = popped.borrow();
                println!("popped={:?}", popped);
                println!("prev={:?}", prev);
                if lvl % 2 == 0 {
                    if popped.val % 2 == 0 {
                        return false;
                    }
                    match prev {
                        Some(p) => {
                            if popped.val < p {
                                return false;
                            }
                        }
                        None => (),
                    }
                } else {
                    if popped.val % 2 == 1 {
                        return false;
                    }
                    match prev {
                        Some(p) => {
                            if popped.val > p {
                                return false;
                            }
                        }
                        None => (),
                    }
                }
                prev = Some(popped.val);
                if let Some(left) = popped.left.clone() {
                    q.push_back(left);
                }
                if let Some(right) = popped.right.clone() {
                    q.push_back(right);
                }
            }
        }
    }
    true
}
