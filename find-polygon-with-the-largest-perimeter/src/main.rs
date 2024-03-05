fn main() {
    println!("Hello, world!");

    let nums = vec![1, 12, 1, 2, 5, 50, 3];
    // Output: 12
    println!("{}", largest_perimeter(nums));
}

pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
    nums.sort_unstable();
    let mut prev_sum = 0;
    let mut rezult = -1;

    for (i, &num) in nums.iter().enumerate() {
        let num = num as i64;
        if i >= 2 {
            if num < prev_sum {
                rezult = prev_sum + num;
            }
        }
        prev_sum += num;
    }

    rezult
}
