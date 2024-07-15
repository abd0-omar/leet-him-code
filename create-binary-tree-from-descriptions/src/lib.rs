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
pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    // descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
    /*
    maybe a hashmap as a graph
    at max a node will have two childrens
    20 | 15,17
    50 | 20,80
    80 | 19
    the reverse of that
    15 | 20
    17 | 20
    20 | 50
    80 | 50
    19 | 80
    that didn't help

    // descriptions = [[20,15,1],[20,17,0],[50,20,1],[50,80,0],[80,19,1]]
    // the root is the node that didn't appear in descriptions[i][1] aka child position

    I think the representation could be a hm with vec
    20 | vec![Some(15), Some(17)]
    80 | vec![Some(19), None]

    once we have that we could now statr constructing
    start from the root and go left
    */

    use std::collections::HashMap;
    use std::collections::HashSet;
    let mut hm = HashMap::new();
    let mut hs_lookup_to_detect_root = HashSet::new();

    for desc in descriptions {
        let parent = desc[0];
        let child = desc[1];
        let is_left = if desc[2] == 1 { true } else { false };
        let f = hm
            .entry(parent)
            // .and_modify(|f: &mut Vec<Option<i32>>| {
            //     println!("{:?}", f);
            //     if is_left {
            //         f[1] = Some(child);
            //     } else {
            //         f[0] = Some(child);
            //     }
            // })
            .or_insert(vec![None; 2]);
        if is_left {
            f[0] = Some(child);
        } else {
            f[1] = Some(child);
        }
        hs_lookup_to_detect_root.insert(child);
    }

    let mut root = None;
    for key in hm.keys() {
        if !hs_lookup_to_detect_root.contains(key) {
            root = Some(Rc::new(RefCell::new(TreeNode::new(*key))));
            break;
        }
    }

    println!("{:?}", hm);
    println!("{:?}", root);

    let tree = root
        .clone()
        .unwrap_or(Rc::new(RefCell::new(TreeNode::new(0))));

    if root.is_some() {
        dfs(&hm, &tree);
    } else {
        return None;
    }

    Some(tree)
}

fn dfs(hm: &std::collections::HashMap<i32, Vec<Option<i32>>>, node: &Rc<RefCell<TreeNode>>) -> () {
    let val = node.borrow().val;
    if let Some(child) = hm.get(&val) {
        let left = child[0];
        let right = child[1];

        if let Some(l) = left {
            let left = Rc::new(RefCell::new(TreeNode::new(l)));
            node.borrow_mut().left = Some(left.clone());
            dfs(hm, &left);
        }
        if let Some(r) = right {
            let right = Rc::new(RefCell::new(TreeNode::new(r)));
            node.borrow_mut().right = Some(right.clone());
            dfs(hm, &right);
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
