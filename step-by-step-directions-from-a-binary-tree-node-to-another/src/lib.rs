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
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
pub fn get_directions(
    root: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
) -> String {
    /*
    it has to be turned into graph
    an adj-list graph
    searching is easy with dfs
    but we need to get the path
    so that if we wanted to do our second search
    any node that was in our first path that means going up
    else in the adj-list I will mark them right and left

    N | L,R
    5 | 1,2
    1 | 3
    3 |
    2 | 6,4
    6 |
    4 |

    N | L,R,U
    5 | 1,2
    1 | 3,N,5
    3 | N,N,1
    2 | 6,4,5
    6 | N,N,2
    4 | N,N,2
    */
    let mut adj_list = HashMap::new();
    let mut q = VecDeque::new();

    if let Some(root_node) = root {
        q.push_back(root_node);
    }

    while let Some(cur_node) = q.pop_front() {
        // neighbors
        if let Some(node) = &cur_node.borrow().left {
            let f = adj_list
                .entry(cur_node.borrow().val)
                .or_insert(vec![None; 3]);
            f[0] = Some(node.borrow().val);

            let f = adj_list.entry(node.borrow().val).or_insert(vec![None; 3]);
            f[2] = Some(cur_node.borrow().val);

            q.push_back(node.clone());
        }
        if let Some(node) = &cur_node.borrow().right {
            let f = adj_list
                .entry(cur_node.borrow().val)
                .or_insert(vec![None; 3]);
            f[1] = Some(node.borrow().val);

            let f = adj_list.entry(node.borrow().val).or_insert(vec![None; 3]);
            f[2] = Some(cur_node.borrow().val);

            q.push_back(node.clone());
        }
    }

    //println!("{:?}", adj_list);

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent_map = HashMap::new();

    q.push_back(start_value);
    visited.insert(start_value);

    while let Some(cur_node) = q.pop_front() {
        if cur_node == dest_value {
            break;
        }

        if let Some(neighbors) = adj_list.get(&cur_node) {
            let left = neighbors[0];
            let right = neighbors[1];
            let up = neighbors[2];

            if let Some(l) = left {
                if !visited.contains(&l) {
                    visited.insert(l);
                    parent_map.insert(l, (cur_node, 'L'));
                    q.push_back(l);
                }
            }
            if let Some(r) = right {
                if !visited.contains(&r) {
                    visited.insert(r);
                    parent_map.insert(r, (cur_node, 'R'));
                    q.push_back(r);
                }
            }
            if let Some(u) = up {
                if !visited.contains(&u) {
                    visited.insert(u);
                    parent_map.insert(u, (cur_node, 'U'));
                    q.push_back(u);
                }
            }
        }
    }

    //println!("parent_map {:?}", parent_map);

    let mut result = String::new();
    let mut cur_node = dest_value;

    while cur_node != start_value {
        if let Some(&(parent, dir)) = parent_map.get(&cur_node) {
            result.push(dir);
            cur_node = parent;
        }
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {}
