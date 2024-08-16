pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    // sort by only a[0] to get the smallest number in each
    // sort by only a[len - 1] to get the biggest number in each
    // add index to avoid edge case that max and min are in the same array

    let n = arrays.len();

    let mut smallest = Vec::with_capacity(n);
    let mut biggest = Vec::with_capacity(n);

    for (i, vec) in arrays.iter().enumerate() {
        smallest.push((vec, i));
        biggest.push((vec, i));
    }

    smallest.sort_unstable_by(|a, b| a.0[0].cmp(&b.0[0]));
    biggest.sort_unstable_by(|b, a| a.0.last().unwrap().cmp(&b.0.last().unwrap()));
    dbg!(&smallest);
    dbg!(&biggest);

    // take the biggest and ignore smallest for the edge case

    let idx_big = 0;
    let mut idx_smol = 0;

    while idx_smol < n {
        if biggest[idx_big].1 == smallest[idx_smol].1 {
            idx_smol += 1;
        } else {
            break;
        }

        // idx_big += 1;
        // idx_smol += 1;
    }

    let potential_max = biggest[idx_big].0.last().unwrap() - smallest[idx_smol].0.first().unwrap();

    let mut idx_big = 0;
    let idx_smol = 0;
    while idx_big < n {
        if biggest[idx_big].1 == smallest[idx_smol].1 {
            idx_big += 1;
        } else {
            break;
        }

        // idx_big += 1;
        // idx_smol += 1;
    }

    let max = potential_max
        .max(biggest[idx_big].0.last().unwrap() - smallest[idx_smol].0.first().unwrap());

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let arrays = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3], vec![3, 6]];
        let output = 5;
        let result = max_distance(arrays);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arrays = vec![vec![1, 4], vec![0, 5]];
        let output = 4;
        let result = max_distance(arrays);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let arrays = vec![vec![2, 4], vec![0, 5]];
        let output = 4;
        let result = max_distance(arrays);
        assert_eq!(result, output);
    }
}
