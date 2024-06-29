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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        _build_tree(&preorder, &inorder, &mut 0, 0, inorder.len() as i32 - 1)
    }
}

fn _build_tree(preorder: &[i32], inorder: &[i32], pre_idx: &mut usize, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if l > r {
        return None;
    }

    let root_val = preorder[*pre_idx];
    *pre_idx += 1;

    let inorder_idx = inorder.iter().position(|&x| x == root_val).unwrap() as i32;

    let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
    root.borrow_mut().left = _build_tree(preorder, inorder, pre_idx, l, inorder_idx - 1);
    root.borrow_mut().right = _build_tree(preorder, inorder, pre_idx, inorder_idx + 1, r);
    Some(root)
}

//building_tree(inorder: &Vec<i32>, l: i32, r: i32, tree: &mut Rc<RefCell<TreeNode>>) -> Option<Rc<RefCell<TreeNode>>>{
    //tree.borrow_mut().left = 
//}
