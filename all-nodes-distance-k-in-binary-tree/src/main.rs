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
pub fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> Vec<i32> {
    let mut graph = vec![vec![]; 500];
    convert_graph(root, &mut graph);
    let mut q = std::collections::VecDeque::new();
    let target_val = target.unwrap().borrow().val;
    q.push_back(target_val);
    let mut lvl = -1;
    let mut visited = std::collections::HashSet::new();
    visited.insert(target_val);
    let mut rez = vec![];

    'outer: while !q.is_empty() {
        lvl += 1;
        let size = q.len();
        for _ in 0..size {
            if lvl == k {
                rez.extend(&q);
                break 'outer;
            }
            let popped = q.pop_front().unwrap();
            if let Some(neighbors) = graph.get(popped as usize) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        q.push_back(neighbor);
                        visited.insert(neighbor);
                    }
                }
            }
        }
    }
    rez
}

fn convert_graph(root: Option<Rc<RefCell<TreeNode>>>, graph: &mut Vec<Vec<i32>>) {
    if let Some(node) = root {
        let node = node.borrow();

        if let Some(l) = node.left.clone() {
            let val = l.borrow().val;
            graph[val as usize].push(node.val);
            graph[node.val as usize].push(val);
        }

        if let Some(r) = node.right.clone() {
            let val = r.borrow().val;
            graph[val as usize].push(node.val);
            graph[node.val as usize].push(val);
        }

        convert_graph(node.left.clone(), graph);
        convert_graph(node.right.clone(), graph);
    }
}
