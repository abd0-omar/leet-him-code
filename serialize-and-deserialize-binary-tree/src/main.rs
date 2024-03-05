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
        let mut count = 0;
        let x = self._serialize(root, &mut count);
        println!("x={:?}", x);
        println!("count={:?}", count);
        x
    }

    fn _serialize(&self, root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> String {
        if let Some(node) = root {
            let node = node.borrow();
            let mut string = format!("({}", node.val);

            string.push(')');

            string.push_str(&self._serialize(node.left.clone(), count));

            string.push_str(&self._serialize(node.right.clone(), count));

            *count += 1;

            return string;
        }
        *count += 1;
        String::from("(x)")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut iter = data.chars().filter_map(|c| match c {
            '0'..='9' => Some(c.to_digit(10).map(|x| x as i32)),
            _ => None,
        });
        println!("iter={:?}", iter);
        self._deserialize(&mut iter)
    }

    fn _deserialize(
        &self,
        data: &mut impl Iterator<Item = Option<i32>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let val = data.next()??;
        let left = self._deserialize(data);
        let right = self._deserialize(data);
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

/*
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
