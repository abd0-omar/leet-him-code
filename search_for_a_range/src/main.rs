fn main() {
    let nums: Vec<i32> = vec![5, 7, 7, 8, 8, 10];
    let target: i32 = 8;

    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    while l + 1 < r {
        let mid = (l + (r - l) / 2) as usize;

        if nums[mid] <= target && nums[mid - 1] == target {
            l = mid as i32;
        }
        if nums[mid] >= target && nums[mid + 1] == target {
            r = mid as i32;
        }
    }

    println!("{}, {}", l, r);
}
