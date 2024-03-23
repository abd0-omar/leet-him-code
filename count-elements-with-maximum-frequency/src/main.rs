fn main() {
    println!("Hello, world!");
}

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();
    for &num in nums.iter() {
        *hm.entry(num).or_insert(0) += 1;
    }
    let (_, max) = hm.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    println!("max={:?}", max);

    let mut count = 0;
    for num in nums { 
        if let Some(s)      = hm.get(&num) {
            if s == max {
                count += 1;
            }
        }
    }

    count
}
