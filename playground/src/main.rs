fn main() {
    let nums = vec![1, 3, 4, 5, 7, 7, 8, 9, 10];
    let target = 7;

    let mut l: usize = 0;
    let mut r = nums.len() - 1;
    let mut ans = -1 as i32;

    while l <= r {
        let mid = l + (r - l) / 2;

        if nums[mid] <= target {
            ans = mid as i32;
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }
    println!("{}, {}, {}", l, r, ans);
}
