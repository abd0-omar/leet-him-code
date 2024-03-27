fn main() {
    println!("Hello, world!");
    let nums = vec![1, 3, 4, 2, 2];
    println!("{}", find_duplicate(nums))
}

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    let mut xor_min_max = 0;
    for i in min..=max {
        println!("i={:?}", i);
        xor_min_max ^= i;
    }
    println!("xor_min_max={:?}", xor_min_max);
    let mut xor_nums = 0;
    for &num in nums.iter() {
        println!("num={:?}", num);
        xor_nums ^= num;
    }
    println!("xor_nums={:?}", xor_nums);
    let xor_all = xor_min_max ^ xor_nums;

    pub fn _find_duplicate(nums: Vec<i32>) -> i32 {
        // Set the pointers
        let mut slow = nums[0];
        let mut fast = nums[0];

        // Loop until slow and fast meet inside loop.
        loop {
            slow = nums[slow as usize];
            println!("slow={:?}", slow);
            fast = nums[nums[fast as usize] as usize];
            println!("fast={:?}", fast);
            if slow == fast {
                break;
            }
        }

        // Set slow to the beginning and run at same speed.
        // This is proven that both pointer will meet at beginning of the loop.
        slow = nums[0];
        while slow != fast {
            slow = nums[slow as usize];
            println!("slow={:?}", slow);
            fast = nums[fast as usize];
            println!("fast={:?}", fast);
        }

        slow
    }
    let f = _find_duplicate(nums);
    f

    // xor_all
}
