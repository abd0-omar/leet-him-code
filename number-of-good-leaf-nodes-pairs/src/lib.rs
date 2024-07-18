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

#[derive(Debug, Clone)]
struct NodeRef(Rc<RefCell<TreeNode>>);

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for NodeRef {}

impl std::hash::Hash for NodeRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

struct Solution;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        // nodes values are not unique so you must implement hash for Rc<RefCell<TreeNode>>
        let mut graph: HashMap<NodeRef, Vec<Option<NodeRef>>> = HashMap::new();
        // graph
        // node | [left, right, parent]
        let mut q = VecDeque::new();
        if let Some(node) = root.clone() {
            q.push_back(NodeRef(node));
        }
        let mut count_leaves: i32 = 0;

        while let Some(cur_node) = q.pop_front() {
            let node_ref = cur_node.0.borrow();
            if node_ref.left.is_none() && node_ref.right.is_none() {
                count_leaves += 1;
            }

            if let Some(n) = node_ref.left.clone() {
                //child
                graph.entry(cur_node.clone()).or_insert(vec![None; 3])[0] =
                    Some(NodeRef(n.clone()));

                //parent
                graph.entry(NodeRef(n.clone())).or_insert(vec![None; 3])[2] =
                    Some(cur_node.clone());

                q.push_back(NodeRef(n));
            }
            if let Some(n) = node_ref.right.clone() {
                //child
                graph.entry(cur_node.clone()).or_insert(vec![None; 3])[1] =
                    Some(NodeRef(n.clone()));

                //parent
                graph.entry(NodeRef(n.clone())).or_insert(vec![None; 3])[2] =
                    Some(cur_node.clone());

                q.push_back(NodeRef(n));
            }
        }

        let mut count: i32 = 0;

        // start from each leaf and then search for the next leaf using bfs
        for (&ref node, neighbors) in graph.iter() {
            if neighbors[0].is_none() && neighbors[1].is_none() {
                // search for the next leaf using bfs
                let mut q = VecDeque::new();
                q.push_back(node);
                let mut visited = HashSet::new();
                let mut lvl = 0;
                while !q.is_empty() {
                    let size = q.len();
                    // size would be at max 2
                    for _ in 0..size {
                        let cur_node = q.pop_front().unwrap();
                        visited.insert(cur_node);

                        if let Some(neighbors) = graph.get(&cur_node) {
                            if neighbors[0].is_none() && neighbors[1].is_none() {
                                // println!("cur_node: {:?}", cur_node);
                                // println!("lvl: {}", lvl);
                                if lvl <= distance {
                                    count += 1;
                                }
                            }
                            if let Some(n) = &neighbors[0] {
                                if !visited.contains(&n) {
                                    q.push_back(&n);
                                }
                            }
                            if let Some(n) = &neighbors[1] {
                                if !visited.contains(&n) {
                                    q.push_back(&n);
                                }
                            }
                            if let Some(n) = &neighbors[2] {
                                if !visited.contains(&n) {
                                    q.push_back(&n);
                                }
                            }
                        }
                    }
                    lvl += 1;
                }
            }
        }
        (count - count_leaves).abs() / 2
    }
}

// fn dfs(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     distance: i32,
//     prev_distance: &mut Option<i32>,
//     cur_distance: i32,
//     count: &mut i32,
// ) -> () {
//     if let Some(node) = root {
//         let node_ref = node.borrow();
//         // check leaf
//         let left_tree = dfs(
//             node_ref.left.clone(),
//             distance,
//             prev_distance,
//             cur_distance + 1,
//             count,
//         );
//         let right_tree = dfs(
//             node_ref.right.clone(),
//             distance,
//             prev_distance,
//             cur_distance + 1,
//             count,
//         );
//         let new_distance = cur_distance + 1;
//         if node_ref.left.is_none() && node_ref.right.is_none() {
//             println!("distance: {:?}", new_distance);
//             println!("node: {:?}", node_ref.val);
//             // check last_prev_distance with cur_distance
//             if let Some(p) = prev_distance {
//                 if (new_distance - *p).abs() <= distance {
//                     *count += 1;
//                     *prev_distance = None
//                 } else {
//                     *prev_distance = Some(new_distance);
//                 }
//             } else {
//                 *prev_distance = Some(new_distance);
//             }
//         }
//     }
// }
#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
