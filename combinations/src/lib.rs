pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    // let mut result_subset = std::collections::HashSet::new();
    let mut result_subset = Vec::new();
    _combine(n, k as usize, vec![], &mut result_subset, 1);
    // println!("result_subset={:?}", result_subset);
    // result_subset.into_iter().collect()
    result_subset
}

//                                  (1, 2, 3, 4)
//                      [1](2, 3, 4)                        (1, 2, 3, 4)
//              [1, 2](3, 4)     [1](2, 3, 4)        [1](2, 3, 4)      (1, 2, 3, 4)

pub fn _combine(
    n: i32,
    k: usize,
    mut cur_subset: Vec<i32>,
    // result_subset: &mut std::collections::HashSet<Vec<i32>>,
    result_subset: &mut Vec<Vec<i32>>,
    st: i32,
) -> () {
    if cur_subset.len() == k {
        // cur_subset.sort_unstable();
        result_subset.push(cur_subset);
        return;
    }

    for end in st..=n {
        cur_subset.push(end);
        _combine(n, k, cur_subset.clone(), result_subset, end + 1);
        cur_subset.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 4;
        let k = 2;
        let output = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        let mut result = combine(n, k);
        result.sort_unstable();
        assert_eq!(result, output);
    }
}
