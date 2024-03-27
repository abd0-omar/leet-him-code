fn main() {
    let nums: Vec<i32> = vec![11, 11];

    let mut l = 0;
    let mut r = nums.len() as i32 - 1;

    let end = nums[nums.len() - 1];
    let mut ans = nums[0];

    while l < r {
        let mid: usize = (l + (r - l) / 2) as usize;

        let am_big: bool = nums[mid] >= end;

        if am_big {
            l = mid as i32 + 1;
            ans = nums[l as usize];
        } else {
            r = mid as i32;
        }
    }

    println!("{}", ans);
}
