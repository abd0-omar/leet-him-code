pub fn count_triplets(arr: Vec<i32>) -> i32 {
    // from i to j not included xor all no.s but not yourself
    // then from j to k included xor all no.s but not yourself
    // if zeror then count++
    //
    // try prefix xor
    // [0, 2, 1, 0, 6, 1]
    // [2, 3, 1, 6, 7]
    // Explanation: The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
    // (0, 2, 2)
    // 2 xor 3 -> 1
    // 1
    // 1 xor 1 -> 0
    //
    // 2 xor 3 -> 1
    // 1

    // (0, 1, 2)
    // 2 xor 2 -> 0
    // 2
    // 3 xor 1 -> 2
    // 2
    // 3 xor 1 -> 2

    let n = arr.len();
    let mut count = 0;
    for i in 0..n {
        let mut xor = 0;
        for j in i + 1..n {
            xor ^= arr[j - 1];
            let mut xor2 = 0;
            for k in j..n {
                xor2 ^= arr[k];
                if xor == xor2 {
                    //println!("xor");
                    //println!("{:?}", &arr[i..=j]);
                    //println!("{:?}", &arr[j..k]);
                    count += 1;
                }
            }
        }
    }

    // let mut prefix_xor = vec![0; n + 1];
    // for i in 1..=n {
    //     prefix_xor[i] = prefix_xor[i - 1] ^ arr[i - 1];
    // }
    // for i in (0..n - 2).rev() {
    //     prefix_xor[i] = prefix_xor[i + 1] ^ arr[i + 1];
    // }
    //println!("prefix_xor={:?}", prefix_xor);

    // prefix xor then search for the third num

    count
}

// a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]
// b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        // don't xor yourself
        // (0, 1, 2)
        // 2 xor 2 -> 0
        // 2
        // 3 xor 1 -> 2
        //
        // (2, 3, 4)
        // 1 xor 1 -> 0
        // 1
        // 6 xor 7 -> 1
        //
        //
        // (0, 2, 2)
        // 2 xor 3 -> 1
        // 1
        // 1 xor 1 -> 0
        //
        // (2, 4, 4)
        // 1 xor 6 -> 7
        // 7
        //            0  1   2  3  4
        let arr = vec![2, 3, 1, 6, 7];
        let output = 4;
        // Explanation: The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
        let result = count_triplets(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![1, 1, 1, 1, 1];
        let output = 10;
        let result = count_triplets(arr);
        assert_eq!(result, output);
    }
}
