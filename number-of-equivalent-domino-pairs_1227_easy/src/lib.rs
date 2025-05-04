// https://leetcode.com/problems/number-of-equivalent-domino-pairs/
#[allow(dead_code)]
struct Solution;

use std::hash::{Hash, Hasher};
#[allow(dead_code)]
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut previous_dominos = std::collections::HashMap::new();
        let mut result = 0;
        for domino in dominoes.into_iter() {
            let domino = Domino::new(&domino);
            if let Some(&cur_count) = previous_dominos.get(&domino) {
                result += cur_count;
            }
            *previous_dominos.entry(domino).or_insert(0) += 1;
        }
        result
    }
}

#[derive(PartialOrd, Ord, Eq)]
struct Domino {
    a: i32,
    b: i32,
}

impl Domino {
    fn new(domino: &[i32]) -> Self {
        Self {
            a: domino[0],
            b: domino[1],
        }
    }
}

impl PartialEq for Domino {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

impl Hash for Domino {
    fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let dominoes = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]];
        let output = 1;
        let result = Solution::num_equiv_domino_pairs(dominoes);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let dominoes = vec![vec![1, 2], vec![1, 2], vec![1, 1], vec![1, 2], vec![2, 2]];
        let output = 3;
        let result = Solution::num_equiv_domino_pairs(dominoes);
        assert_eq!(result, output);
    }
}
