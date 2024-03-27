fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;

    let mut l: i32 = 0;
    let mut r: i32 = nums.len() as i32 - 1;

    let mut ans: i32 = -1;
    let first = nums[0];

    while l <= r {
        let mid = l + (r - l) / 2;

        if nums[mid as usize] == target {
            ans = mid as i32;
            break;
        }

        let am_big: bool = nums[mid as usize] >= first;
        let target_big: bool = target >= first;

        if am_big == target_big {
            if nums[mid as usize] < target {
                l = mid + 1;
            } else if nums[mid as usize] > target {
                r = mid - 1;
            }
        } else {
            if nums[mid as usize] > target {
                l = mid + 1;
            } else if nums[mid as usize] < target {
                r = mid - 1;
            }
        }
    }
    println!("{}", ans);
}
