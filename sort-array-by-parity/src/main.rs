fn main() {
    let nums = vec![3, 1, 2, 4];
    let output = vec![2, 4, 3, 1];
    let nums2 = vec![3, 1, 2, 4, 5, 7];
    println!("{:?}", sort_array_by_parity(nums));
}

pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut nums_odd = nums.iter().filter(|&x| x % 2 == 1).collect::<Vec<_>>();
    let mut nums_even = nums.iter().filter(|x| **x % 2 == 0).collect::<Vec<_>>();
    println!("DEBUGPRINT[2]: main.rs:6: nums_odd={:#?}", nums_odd);
    println!("DEBUGPRINT[1]: main.rs:7: nums_even={:#?}", nums_even);

    nums_even.extend_from_slice(&nums_odd);
    println!("DEBUGPRINT[3]: main.rs:14: nums_odd={:#?}", nums_even);

    let nums_even = nums_even.iter().map(|x| **x).collect::<Vec<i32>>();
    nums_even
}
