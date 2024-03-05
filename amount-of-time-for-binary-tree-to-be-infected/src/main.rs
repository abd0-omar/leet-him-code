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
pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let mut graph = std::collections::HashMap::new();
    convert_graph(root, &mut graph);
    println!("graph={:?}", graph);
    let mut q = std::collections::VecDeque::new();
    q.push_back(start);
    let mut lvl = -1;
    let mut visited = std::collections::HashSet::new();
    visited.insert(start);
    while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            let popped = q.pop_front().unwrap();
            if let Some(neighbors) = graph.get(&popped) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        q.push_back(neighbor);
                    }
                }
            }
        }
    }
    lvl
}

fn convert_graph(
    root: Option<Rc<RefCell<TreeNode>>>,
    graph: &mut std::collections::HashMap<i32, Vec<i32>>,
) {
    if let Some(node) = root {
        let node = node.borrow();
        if let Some(l) = node.left.clone() {
            graph.entry(l.borrow().val).or_insert(vec![]).push(node.val);
            graph.entry(node.val).or_insert(vec![]).push(l.borrow().val);
        }

        if let Some(r) = node.right.clone() {
            graph.entry(r.borrow().val).or_insert(vec![]).push(node.val);
            graph.entry(node.val).or_insert(vec![]).push(r.borrow().val);
        }
        convert_graph(node.left.clone(), graph);
        convert_graph(node.right.clone(), graph);
    }
}
