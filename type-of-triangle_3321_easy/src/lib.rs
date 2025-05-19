// https://leetcode.com/problems/type-of-triangle/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort_unstable();
        Triangle::parse(nums[0], nums[1], nums[2]).to_string()
    }
}

enum Triangle {
    Equilateral,
    Isosceles,
    Scalene,
    None,
}

impl Triangle {
    fn parse(x: i32, y: i32, z: i32) -> Self {
        if x + y <= z {
            return Self::None;
        }
        if x == y && y == z {
            return Self::Equilateral;
        }
        if x == y || x == z || y == z {
            return Self::Isosceles;
        }
        Self::Scalene
    }
}

impl std::fmt::Display for Triangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Triangle::Equilateral => write!(f, "equilateral"),
            Triangle::Isosceles => write!(f, "isosceles"),
            Triangle::Scalene => write!(f, "scalene"),
            Triangle::None => write!(f, "none"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 3, 3];
        let output = "equilateral".to_string();
        let result = Solution::triangle_type(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![3, 4, 5];
        let output = "scalene".to_string();
        let result = Solution::triangle_type(nums);
        assert_eq!(result, output);
    }
}
