pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    // pure brute force would be
    // to iterate on arr2 then iterate on arr1 to find the element and push to result vec
    // (nested loop)
    // let arr2 = vec![2, 1, 4, 3, 96];
    //                 i ->
    // let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    //                 j ->
    // and it would work because of small constraints
    //
    // optimized solution hashmap/freq_array
    //
    // make a freq array of arr1 (because small input, freq_array better as hashing takes a small
    // time)
    // iterate on arr2 and push the no. of occurences of the element in result

    let mut freq = vec![0; 1001];
    for &element in arr1.iter() {
        freq[element as usize] += 1;
    }

    let mut result = Vec::with_capacity(arr1.len());

    for element in arr2 {
        for _ in 0..freq[element as usize] {
            result.push(element);
        }
        freq[element as usize] = 0;
    }

    // add the rest of arr1
    for i in 0..freq.len() {
        for _ in 0..freq[i] {
            result.push(i as i32);
            freq[i] -= 1;
        }
    }

    // rust way
    // let mut hm = std::collections::HashMap::new();
    // for (idx, element) in arr2.iter().enumerate() {
    //     hm.insert(element, idx);
    // }
    //
    // let mut result = arr1.clone();
    //
    // result.sort_unstable_by(|&a, &b| match (hm.get(&a), hm.get(&b)) {
    //     (None, None) => a.cmp(&b),
    //     (None, Some(_)) => std::cmp::Ordering::Greater,
    //     (Some(_), None) => std::cmp::Ordering::Less,
    //     (Some(idx1), Some(idx2)) => idx1.cmp(idx2),
    // });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
        let arr2 = vec![2, 1, 4, 3, 9, 6];
        let output = vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19];
        let result = relative_sort_array(arr1, arr2);
        assert_eq!(result, output);
    }
}
