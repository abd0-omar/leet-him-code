fn main() {
    println!("Hello, world!");
}

pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let max = *nums.iter().max().unwrap_or(&0);
    let mut freq = vec![0; (max + 1) as usize];
    let mut rezult = vec![];
    for num in nums.into_iter() {
        if freq[num as usize] == 1 {
            rezult.push(num);
        }
        freq[num as usize] += 1;
    }
    rezult
}
