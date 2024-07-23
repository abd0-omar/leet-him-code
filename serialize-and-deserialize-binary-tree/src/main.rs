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
struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let f = self._serialize(root);
        // println!("{}", f); // (1(2()())(3(4()())(5()())))
        f
    }

    fn _serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            let node = node.borrow();
            let mut s = format!("({}", node.val);

            if let Some(n) = node.left.clone() {
                s.push_str(&self._serialize(Some(n)));
            } else {
                s.push_str("()");
            }

            if let Some(n) = node.right.clone() {
                s.push_str(&self._serialize(Some(n)));
            } else {
                s.push_str("()");
            }

            s.push(')');

            return s;
        }
        String::from("()")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut index = 0;
        self._deserialize(&data, &mut index)
    }

    fn _deserialize(&self, data: &str, index: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if *index >= data.len() || &data[*index..*index + 2] == "()" {
            *index += 2; // Skip over the "()" for an empty node
            return None;
        }

        // Skip the opening '('
        *index += 1;
        // Parse the node's value
        let mut val = 0;
        let mut negative = false;
        if data.as_bytes()[*index] == b'-' {
            negative = true;
            *index += 1;
        }
        while *index < data.len() && (data.as_bytes()[*index] as char).is_digit(10) {
            val = val * 10 + (data.as_bytes()[*index] as char).to_digit(10).unwrap() as i32;
            *index += 1;
        }
        if negative {
            val = -val;
        }

        let node = Rc::new(RefCell::new(TreeNode::new(val)));

        node.borrow_mut().left = self._deserialize(data, index);
        node.borrow_mut().right = self._deserialize(data, index);

        // Skip the closing ')'
        *index += 1;

        Some(node)
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
