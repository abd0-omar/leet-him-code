fn main() {
    println!("Hello, world!");
}
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut rezult = nums.clone();

    for rezz in rezult.iter_mut() {
        *rezz = rezz.pow(2);
    }
    rezult.sort_unstable();
    rezult
}
