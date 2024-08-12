use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    pq: BinaryHeap<Reverse<i32>>,
    max: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        /*
                Example 1:

        Input
        ["KthLargest", "add", "add", "add", "add", "add"]
        [[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]]
        Output
        [null, 4, 5, 5, 8, 8]

        Explanation
        KthLargest kthLargest = new KthLargest(3, [4, 5, 8, 2]);
        kthLargest.add(3);   // return 4
        kthLargest.add(5);   // return 5
        kthLargest.add(10);  // return 5
        kthLargest.add(9);   // return 8
        kthLargest.add(4);   // return 8
        */
        /*
        2, 4, 5, 8

              x

        10, 9, 8, top

        8, 5, 4

        // if cur < top, don't add it to the binary heap
        // else pop and add
        // max len
        //
        //
        // 0, -1
        // 1, 0
        */
        let mut pq = BinaryHeap::new();
        for num in nums {
            pq.push(Reverse(num));
        }

        while pq.len() as i32 > k {
            pq.pop();
        }

        dbg!(&pq);

        Self {
            pq,
            max: k as usize,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        dbg!(val);
        dbg!(&self.pq);
        if self.pq.len() < self.max {
            self.pq.push(Reverse(val));
        } else {
            if let Some(top) = self.pq.peek() {
                if val > top.0 {
                    self.pq.pop();
                    self.pq.push(Reverse(val));
                }
            } else {
                self.pq.push(Reverse(val));
            }
        }
        dbg!("after");
        dbg!(&self.pq);

        self.pq.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let k = 3;
        let nums = vec![4, 5, 8, 2];
        let mut kth = KthLargest::new(k, nums);
        let ret_1 = kth.add(3);
        assert_eq!(ret_1, 4);
        let ret_1 = kth.add(5);
        assert_eq!(ret_1, 5);
        let ret_1 = kth.add(10);
        assert_eq!(ret_1, 5);
        let ret_1 = kth.add(9);
        assert_eq!(ret_1, 8);
        let ret_1 = kth.add(4);
        assert_eq!(ret_1, 8);
    }

    #[test]
    fn it_works1() {
        let k = 1;
        let nums = vec![];
        let mut kth = KthLargest::new(k, nums);
        // wrong asserts, just for quick input testing
        let ret_1 = kth.add(-3);
        assert_eq!(ret_1, 4);
        let ret_1 = kth.add(-2);
        assert_eq!(ret_1, 5);
        let ret_1 = kth.add(-4);
        assert_eq!(ret_1, 5);
        let ret_1 = kth.add(0);
        assert_eq!(ret_1, 8);
        let ret_1 = kth.add(4);
        assert_eq!(ret_1, 8);
    }

    #[test]
    fn it_works2() {
        let k = 2;
        let nums = vec![0];
        let mut kth = KthLargest::new(k, nums);
        let ret_1 = kth.add(-1);
        assert_eq!(ret_1, -1);
        let ret_1 = kth.add(1);
        assert_eq!(ret_1, 0);
        let ret_1 = kth.add(-2);
        assert_eq!(ret_1, 0);
        let ret_1 = kth.add(-4);
        assert_eq!(ret_1, 0);
        let ret_1 = kth.add(3);
        assert_eq!(ret_1, 1);
    }
}
