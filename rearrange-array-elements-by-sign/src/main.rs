fn main() {
    let nums = vec![3, 1, -2, -5, 2, -4];
    // Output: [3,-2,1,-5,2,-4]
    println!("{:?}", rearrange_array(nums));
}

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut pos_arr = Vec::with_capacity(n / 2);
    let mut neg_arr = Vec::with_capacity(n / 2);
    let mut rezult = Vec::with_capacity(n);

    for num in nums.iter() {
        if num.is_positive() {
            pos_arr.push(num);
        } else {
            neg_arr.push(num);
        }
    }

    let mut pos_idx = 0;
    let mut neg_idx = 0;
    for i in 0..nums.len() {
        if i % 2 == 0 {
            rezult.push(pos_arr[pos_idx]);
            pos_idx += 1;
        } else {
            rezult.push(neg_arr[neg_idx]);
            neg_idx += 1;
        }
    }

    rezult.iter().map(|&x| *x).collect()
}
