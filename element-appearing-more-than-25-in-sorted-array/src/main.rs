fn main() {
    println!("Hello, world!");
    // Example 1:

    let arr = vec![1, 2, 2, 6, 6, 6, 6, 7, 10];
    // Output: 6
    find_special_integer(arr);
}

pub fn find_special_integer(arr: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();
    for a in arr {
        *hm.entry(a).or_insert(0) += 1;
    }
    let x = hm.iter().map(|x| (x.1, x.0)).max().unwrap();
    println!("x={:?}", x);
    *x.1
}
