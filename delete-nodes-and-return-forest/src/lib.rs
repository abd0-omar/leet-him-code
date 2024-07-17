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
pub fn del_nodes(
    root: Option<Rc<RefCell<TreeNode>>>,
    to_delete: Vec<i32>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    /*
    first thought to convert it to graph and it's now becomes a simple
    union-find (dsu)
    and I think it's time complexity is O(n) so yeah maybe it'll work
    yeah there is maybe a better dfs solution but who cares
    */
    // use std::collections::{HashMap, HashSet, VecDeque};
    // let mut graph = HashMap::new();
    // // preorder(root, &mut graph);
    // // bfs is better
    // let mut q = VecDeque::new();
    // if let Some(node) = root {
    //     q.push_back(node);
    // }
    // while let Some(cur_node) = q.pop_front() {
    //     let cur_node = cur_node.borrow();
    //     let f = graph.entry(cur_node.val).or_insert(vec![None; 2]);
    //     if let Some(n) = cur_node.left.clone() {
    //         f[0] = Some(n.borrow().val);
    //         q.push_back(n);
    //     }
    //     if let Some(n) = cur_node.right.clone() {
    //         f[1] = Some(n.borrow().val);
    //         q.push_back(n);
    //     }
    // }
    // println!("{:?}", graph);
    use std::collections::{HashMap, HashSet, VecDeque};
    let mut hs = HashSet::new();
    for del in to_delete {
        hs.insert(del);
    }
    let mut trees = Vec::new();
    let root = postorder(root, &hs, &mut trees);

    if root.is_some() {
        trees.push(root);
    }

    trees
    // dfs is more logical to think about it but the bfs way is more intuitive in terms of code in my opinion
}

fn postorder(
    root: Option<Rc<RefCell<TreeNode>>>,
    hs: &std::collections::HashSet<i32>,
    trees: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let mut node_borrow = node.borrow_mut();
        node_borrow.left = postorder(node_borrow.left.take(), hs, trees);
        node_borrow.right = postorder(node_borrow.right.take(), hs, trees);

        if hs.contains(&node_borrow.val) {
            if node_borrow.left.is_some() {
                trees.push(node_borrow.left.take());
            }
            if node_borrow.right.is_some() {
                trees.push(node_borrow.right.take());
            }
            // dont return the deleted node
            return None;
        }
        return Some(node.clone());
    }
    None
}

struct UnionFind {
    parents: Vec<i32>,
    ranks: Vec<i32>,
    forests: i32,
}

impl UnionFind {
    fn new(n: i32) -> Self {
        // every node is it's own parent
        let parents = (1..=n).collect();
        // every rank is 1
        let ranks = vec![1; n as usize];
        Self {
            parents,
            ranks,
            forests: n,
        }
    }
}

// fn preorder(
//     root: Option<Rc<RefCell<TreeNode>>>,
//     graph: &mut std::collections::HashMap<i32, Vec<Option<i32>>,
// ) -> () {
//     if let Some(node) = root {
//         let node = node.borrow();
//         preorder(node.left.clone(), graph);
//         preorder(node.right.clone(), graph);
//         let f = graph.entry(node.val).or_insert(vec![]);

//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
