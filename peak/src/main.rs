fn main() {
    let nums: Vec<i32> = vec![3, 2, 1];

    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    let mut ans = 0;

    while l <= r {
        let mid: usize = (l + (r - l) / 2) as usize;

        if mid > 0 && nums[mid] >= nums[mid - 1] {
            ans = mid as i32;
            l = mid as i32 + 1;
        } else if mid > 0 && nums[mid] < nums[mid - 1] {
            r = mid as i32 - 1;
        }
    }
    println!("{ans}");
}
