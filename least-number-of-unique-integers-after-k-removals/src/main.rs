fn main() {
    println!("Hello, world!");
}

pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
    let mut hm = std::collections::HashMap::new();
    for &a in arr.iter() {
        *hm.entry(a).or_insert(0) += 1;
    }

    let mut sorted_hm = hm.iter().collect::<Vec<_>>();
    sorted_hm.sort_by(|a, b| a.1.cmp(b.1));
    // println!("sorted_hm={:?}", sorted_hm);

    let mut count = k;
    for (i, &pair) in sorted_hm.iter().enumerate() {
        count -= *pair.1;
        if count == 0 {
            return hm.len() as i32 - i as i32 - 1;
        } else if count < 0 {
            return hm.len() as i32 - i as i32;
        }
    }

    hm.len() as i32
}

#[test]
fn name() {
    let arr = vec![4, 3, 1, 1, 3, 3, 2];
    let k = 3;
    // Output: 2
    // Explanation: Remove 4, 2 and either one of the two 1s or three 3s. 1 and 3 will be left.
    assert_eq!(
        2,
        find_least_num_of_unique_ints(arr, k),
        "testing cuz why not"
    );
}

#[test]
fn name2() {
    let arr = vec![5, 5, 4];
    let k = 1;
    assert_eq!(
        1,
        find_least_num_of_unique_ints(arr, k),
        "testing cuz why not"
    );
}
