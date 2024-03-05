fn main() {
    println!("Hello, world!");
}

pub fn is_monotonic(nums: Vec<i32>) -> bool {
    let (mut ascen, mut descen): (bool, bool) = (true, true);

    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            descen = false;
        } else if nums[i] < nums[i - 1] {
            ascen = false;
        }
    }

    ascen || descen
}
