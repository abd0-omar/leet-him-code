// https://leetcode.com/problems/product-of-the-last-k-numbers/
#[allow(dead_code)]
struct Solution;

struct ProductOfNumbers {
    prefix_mult: Vec<i32>,
    last_zero_index: Option<usize>,
    count: usize, // redundant but helps instead of using prefix_mult.len() - 1
}

//   [3,0,2,5,4]
// [1,3,1,2,10,40]

impl ProductOfNumbers {
    fn new() -> Self {
        Self {
            prefix_mult: vec![1],
            last_zero_index: None,
            count: 0,
        }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.prefix_mult = vec![1];
            self.last_zero_index = Some(self.count);
        } else {
            let last_product = *self.prefix_mult.last().unwrap();
            self.prefix_mult.push(last_product * num);
        }
        self.count += 1;
    }

    fn get_product(&self, k: i32) -> i32 {
        let k_usize = k as usize;

        if let Some(zero_idx) = self.last_zero_index {
            if zero_idx >= self.count - k_usize {
                return 0;
            }
        }

        let len = self.prefix_mult.len();
        self.prefix_mult[len - 1] / self.prefix_mult[len - k_usize - 1]
    }
}
