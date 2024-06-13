fn main() {
    println!("Hello, world!");
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = std::collections::HashSet::new();
    for i in 0..nums.len() {
        let mut hm = std::collections::HashMap::new();
        for j in i + 1..nums.len() {
            // - 1, 0
            // and I'm searching for 0
            // so we need a 1
            let diff = -(nums[i] + nums[j]);
            println!("{:?}", hm);

            if j != i + 1 {
                if let Some(&k) = hm.get(&diff) {
                    println!("nums[i]: {i}");
                    println!("i: {i}");
                    println!("nums[j]: {j}");
                    println!("j: {j}");
                    println!("nums[k]: {k}");
                    println!("k: {k}");
                    let mut temp = vec![nums[i], nums[j], nums[k]];
                    temp.sort_unstable();
                    result.insert(temp);
                }
            }

            hm.insert(nums[j], j);
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::three_sum;

    #[test]
    fn it_works() {
        // hm {1, 0, -1}
        // -1, 0, 1, 2, -1, -4
        //  i  j (1)
        //  i     j (2)
        //  i        j (3)
        //  i
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let result = three_sum(nums);
        assert_eq!(result, output);
    }
}
