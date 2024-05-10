use std::cmp::{Ordering, Reverse};

pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Fraction {
        value: Reverse<f64>,
        numerator_index: usize,
        denominator_index: usize,
    }

    impl Eq for Fraction {}
    impl Ord for Fraction {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value
                .partial_cmp(&other.value)
                .unwrap_or(Ordering::Equal)
        }
    }
    let mut pq = std::collections::BinaryHeap::new();
    for i in 0..arr.len() {
        for j in i + 1..arr.len() {
            let value = arr[i] as f64 / arr[j] as f64;
            pq.push(Fraction {
                value: Reverse(value),
                numerator_index: i,
                denominator_index: j,
            });
        }
    }
    println!("pq={:?}", pq);

    // Pop the kth smallest fraction
    let mut result = vec![];
    for i in 0..k {
        if let Some(fraction) = pq.pop() {
            if i == k - 1 {
                result = vec![
                    arr[fraction.numerator_index],
                    arr[fraction.denominator_index],
                ];
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![1, 2, 3, 5];
        let k = 3;
        let output = vec![2, 5];
        let result = kth_smallest_prime_fraction(arr, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![1, 7];
        let k = 1;
        let output = vec![1, 7];
        let result = kth_smallest_prime_fraction(arr, k);
        assert_eq!(result, output);
    }
}
