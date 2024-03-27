fn main() {
    let nums: Vec<i32> = vec![-1, 0, 3, 5, 9, 12];
    let target: i32 = 9;

    let mut st: i32 = 0;
    let mut end: i32 = nums.len() as i32 - 1;

    while st <= end {
        let mid: i32 = st + end / 2;

        if nums[mid as usize] == target {
            println!("{mid}");
        } else if nums[mid as usize] > target {
            st = mid + 1;
        } else {
            end = mid - 1;
        }
    }
}
