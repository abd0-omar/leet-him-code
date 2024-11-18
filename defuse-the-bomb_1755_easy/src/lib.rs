// https://leetcode.com/problems/defuse-the-bomb/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decrypt(code: Vec<i32>, mut k: i32) -> Vec<i32> {
        // yoo is that a bomb
        // حد هنا عارف في البرنجة
        // ايوة انا في سنة رابعة في كلية حاسب لتقع
        // ايه المشكل؟
        // ينهار ابيض
        // كله يبعد عن  eام القنبلة مبهزرش دلوقتي
        // ونولوني تيرمينال
        // for (int i = 0; i >= 5; i++)

        // brute-force is trivial, but I wonder what is the optimal solution
        let n = code.len();
        let mut result = vec![0; n];

        let k_status = match k {
            1.. => KStatus::Positive,
            i32::MIN..=-1 => {
                k = k * -1;
                KStatus::Negative
            }
            _ => return result,
        };

        dbg!(&k_status);
        for i in 0..n {
            let mut cur_sum = 0;
            for j in 0..k {
                match k_status {
                    KStatus::Positive => cur_sum += code[(i + 1 + j as usize) % n],
                    KStatus::Negative => {
                        cur_sum += code[((i as i32 - 1 - j + n as i32/*add `n` to avoid negative mod*/)
                            % n as i32) as usize]
                    }
                }
                // dbg!(((i as i32 - 1 - j) % n as i32) as usize);
            }
            // dbg!(&cur_sum);
            result[i] = cur_sum;
        }

        result
    }
}

#[derive(Debug)]
enum KStatus {
    Positive,
    Negative,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let code = vec![5, 7, 1, 4];
        let k = 3;
        let output = vec![12, 10, 16, 13];
        let result = Solution::decrypt(code, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let code = vec![1, 2, 3, 4];
        let k = 0;
        let output = vec![0, 0, 0, 0];
        let result = Solution::decrypt(code, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let code = vec![2, 4, 9, 3];
        let k = -2;
        let output = vec![12, 5, 6, 13];
        let result = Solution::decrypt(code, k);
        assert_eq!(result, output);
    }
}
