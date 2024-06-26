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
pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    // parse to an array
    let mut sorted_tree = Vec::new();
    bst_to_array(root, &mut sorted_tree);
    // println!("{:?}", sorted_tree);

    // construct balanced bst from sorted array using binary search
    let balanced_tree = construct_bst(&sorted_tree, 0, sorted_tree.len() as i32 - 1);
    // println!("{:?}", balanced_tree);
    balanced_tree
}

fn construct_bst(sorted_tree: &Vec<i32>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
    // while l <= r
    if l > r {
        return None;
    }

    let mid = l + (r - l) / 2;

    let left_tree = construct_bst(sorted_tree, l, mid - 1);
    let right_tree = construct_bst(sorted_tree, mid + 1, r);
    let root = Rc::new(RefCell::new(TreeNode::new(sorted_tree[mid as usize])));

    root.borrow_mut().left = left_tree;
    root.borrow_mut().right = right_tree;

    Some(root)
}

fn bst_to_array(root: Option<Rc<RefCell<TreeNode>>>, sorted_tree: &mut Vec<i32>) -> () {
    if let Some(node) = root {
        let node = node.borrow();
        // inorder traversal
        bst_to_array(node.left.clone(), sorted_tree);
        sorted_tree.push(node.val);
        bst_to_array(node.right.clone(), sorted_tree);
    }
}

#[cfg(test)]
mod tests {}
