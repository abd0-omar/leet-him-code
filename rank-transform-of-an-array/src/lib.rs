pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    // Leetcode transform ranks
    // 40, 10, 20, 30, 10
    // 4,    1,   2,    3,   1
    //
    // Put in tuples
    // (40,0) the sort
    //
    // 40, 10, 20, 30, 10
    // (10, 1), (10, 4), (20, 2), (30, 3), (40, 0)
    //
    // ranks: -1, -1...
    // Assign ranks
    // Count = 1
    // If i !=0 && arr.0 == arr.0 before
    // Dont increment count and just assign
    //
    // Else
    // Assign and increment count
    //
    //
    // -1, 1...
    // -1, 1, -1, -1, 1
    // -1, 1, 2, -1, 1
    //
    // -1, 1, 2, 3, 1
    //
    // 4, 1, 2, 3, 1
    // This steps was written on my phone till I go home and write the solution
    //
    // Could be done with a hashmap too, but the keys are not unique so we would've done a work around for that
    let n = arr.len();
    let mut arr_tuple_idx: Vec<_> = arr
        .into_iter()
        .enumerate()
        .map(|(idx, x)| (x, idx))
        .collect();
    arr_tuple_idx.sort_unstable();
    let mut ranks = vec![-1; n];
    let mut count = 1;
    for i in 0..n {
        if i != n - 1 && arr_tuple_idx[i].0 == arr_tuple_idx[i + 1].0 {
            // don't increment count
            ranks[arr_tuple_idx[i].1] = count;
        } else {
            ranks[arr_tuple_idx[i].1] = count;
            count += 1;
        }
        dbg!(&ranks);
    }
    ranks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![40, 10, 20, 30, 10];
        let output = vec![4, 1, 2, 3, 1];
        let result = array_rank_transform(arr);
        assert_eq!(result, output);
    }
}
