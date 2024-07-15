// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
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
pub fn sorted_list_to_bst(mut head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut vec = vec![];
    while let Some(cur) = head.take() {
        let nxt = cur.next;
        vec.push(cur.val);
        head = nxt;
    }
    //println!("{:?}", vec);

    if vec.is_empty() {
        return None;
    }

    let l = 0;
    let r = vec.len() as i32 - 1;
    let tree = build(l, r, &vec);
    tree
}

fn build(l: i32, r: i32, vec: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    // while l <= r
    if l > r {
        return None;
    }
    let mid = l + (r - l) / 2;

    let new_node = Rc::new(RefCell::new(TreeNode::new(vec[mid as usize])));

    new_node.borrow_mut().left = build(l, mid - 1, vec);
    new_node.borrow_mut().right = build(mid + 1, r, vec);
    Some(new_node)
}

#[cfg(test)]
mod tests {}
