fn main() {
    println!("Hello, world!");

    // Create a binary tree: [4, 2, 6]
    // let root = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 4,
    //     left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
    //     right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
    // })));
    // println!("{:?}", average_of_subtree(root));
    // Create the binary tree: [4,8,5,0,1,null,6]
    // let root = Some(Rc::new(RefCell::new(TreeNode {
    //     val: 4,
    //     left: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 8,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
    //         right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    //     }))),
    //     right: Some(Rc::new(RefCell::new(TreeNode {
    //         val: 5,
    //         left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
    //         right: None,
    //     }))),
    // })));
    //
    // let averages = average_of_subtree(root);
    // println!("{:?}", averages);
    // Create the binary tree: [1,null,3,null,1,null,3]
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        }))),
    })));
    println!("{:?}", average_of_subtree(root));
}

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
pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut v = Vec::new();
    let _ = _vec_of_avgs(root, &mut v);
    println!("{:?}", v);
    let mut count = 0;
    for (node, avg) in v {
        if node == avg {
            count += 1;
        }
    }
    count
}

fn _vec_of_avgs(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<(i32, i32)>) -> (i32, i32) {
    if let Some(node) = root {
        let node = node.borrow();
        let count = 1;
        let n_val = node.val;

        let (mut l_sum, mut l_count) = _vec_of_avgs(node.left.clone(), v);

        if let Some(l) = node.left.clone() {
            l_sum += l.borrow().val;
            l_count += 1;
        }

        let (mut r_sum, mut r_count) = _vec_of_avgs(node.right.clone(), v);

        if let Some(r) = node.right.clone() {
            r_sum += r.borrow().val;
            r_count += 1;
        }

        let avg = (l_sum + r_sum + n_val) / (count + l_count + r_count);

        v.push((node.val, avg));
        return (l_sum + r_sum + n_val, count + l_count + r_count);
    }
    (0, 0)
}
