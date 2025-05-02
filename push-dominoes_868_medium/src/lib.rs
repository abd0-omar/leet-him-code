// https://leetcode.com/problems/push-dominoes/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        // first idea to come to mind is bfs
        let mut dominoes = dominoes.chars().collect::<Vec<char>>();
        let mut q = std::collections::VecDeque::new();
        for (i, &c) in dominoes.iter().enumerate() {
            if c != '.' {
                q.push_back((i, c));
            }
        }
        let mut temp = Vec::new();
        let n = dominoes.len();
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let (i, c) = q.pop_front().unwrap();
                if c == 'L' {
                    // L.R
                    if i as i32 - 2 >= 0 && dominoes[i - 2] == 'R' {
                        continue;
                    }
                    if i > 0 && dominoes[i - 1] == '.' {
                        q.push_back((i - 1, 'L'));
                        temp.push((i - 1, 'L'));
                    }
                } else if c == 'R' {
                    // R.L
                    if i + 2 < n && dominoes[i + 2] == 'L' {
                        continue;
                    }
                    if i < dominoes.len() - 1 && dominoes[i + 1] == '.' {
                        q.push_back((i + 1, 'R'));
                        temp.push((i + 1, 'R'));
                    }
                }
            }
            // after a level of bfs is done update the original string
            for &(i, c) in temp.iter() {
                dominoes[i] = c;
            }
        }
        return dominoes.iter().collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let dominoes = "RR.L".to_string();
        let output = "RR.L".to_string();
        let result = Solution::push_dominoes(dominoes);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let dominoes = ".L.R...LR..L..".to_string();
        let output = "LL.RR.LLRRLL..".to_string();
        let result = Solution::push_dominoes(dominoes);
        assert_eq!(result, output);
    }
}
