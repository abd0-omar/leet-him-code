fn main() {
    println!("Hello, world!");
    // Example 1:
    //
    let arr = vec![3, 1, 2, 4];
    println!("{}", sum_subarray_mins(arr));
    // Output: 17
    // Explanation:
    // Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4].
    // Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
    // Sum is 17.
}

pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let mut vec = Vec::new();
    for start in 0..arr.len() {
        for end in start..arr.len() {
            let mut v = Vec::new();
            for j in start..=end {
                v.push(arr[j]);
            }
            vec.push(v);
        }
    }
    println!("{:?}", vec);
    0
}
