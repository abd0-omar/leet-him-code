fn main() {
    println!("Hello, world!");
    let nums = vec![0, 0, 1, 0, 0, 0, 1, 1];
    println!("{}", find_max_length(nums));
}

pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    // let mut prefix_sum = vec![0; n + 1];
    let mut hm = std::collections::HashMap::from([(0, 0)]);
    let mut cur_sum = 0;
    let mut maximus = 0;
    for i in 1..=n {
        let num = if nums[i - 1] == 1 { 1 } else { -1 };
        cur_sum = cur_sum + num;
        println!("cur_sum={:?}", cur_sum);
        if let Some(idx) = hm.get(&cur_sum) {
            println!("idx={:?}", idx);
            maximus = maximus.max(i - idx);
        } else {
            hm.insert(cur_sum, i);
        };
    }
    println!("hm={:?}", hm);
    maximus as _
}
