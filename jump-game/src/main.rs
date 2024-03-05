fn main() {
    println!("Hello, world!");
    let nums = vec![3, 2, 1, 0, 4];
    // Output: false
    println!("{}", can_jump(nums));
    // let nums = vec![2, 1, 0, 1, 4];
    // // Output: true
    // println!("{}", can_jump(nums));
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut reachable = vec![false; nums.len()];
    reachable[0] = true;
    for i in 0..nums.len() {
        println!("i={:?}", i);
        println!("num={:?}", nums[i]);
        for neighbor in get_neigbors(&nums, i) {
            let num = i + neighbor as usize;
            if num < nums.len() {
                println!("neighbor={:?}", nums[i + neighbor as usize]);
            } else {
                println!("out of bounds");
                continue;
            }

            let real_num = i + neighbor as usize;
            println!("neighbor real num={:?}", real_num);
            if reachable[i] {
                reachable[real_num] = true;
            }
            println!("reachable={:?}", reachable);
        }
    }
    println!("reachable={:?}", reachable);
    if reachable[nums.len() - 1] == true {
        true
    } else {
        false
    }
}

fn get_neigbors(nums: &Vec<i32>, i: usize) -> impl Iterator<Item = i32> {
    let mut vec = Vec::new();
    let mut idx = nums[i];
    println!("idx={:?}", idx);
    while idx > 0 {
        vec.push(idx);
        idx -= 1;
    }
    println!("vec={:?}", vec);
    vec.into_iter()
}
