// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[allow(dead_code)]
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        let mut matrix = vec![vec![-1; n]; m];

        let mut direction = Direction::Right;

        let mut i = 0;
        let mut j = 0;

        let mut cur_head = head.as_ref();
        while let Some(cur) = cur_head {
            matrix[i][j] = cur.val;
            if i == m && j == n - 1 || i == m - 1 && j == n {
                break;
            }

            match direction {
                Direction::Right => {
                    if j == n - 1 || matrix[i][j + 1] != -1 {
                        direction = Direction::Down;
                        i += 1;
                    } else {
                        j += 1;
                    }
                }
                Direction::Down => {
                    if i == m - 1 || matrix[i + 1][j] != -1 {
                        direction = Direction::Left;
                        j -= 1;
                    } else {
                        i += 1;
                    }
                }
                Direction::Left => {
                    if j == 0 || matrix[i][j - 1] != -1 {
                        direction = Direction::Up;
                        i -= 1;
                    } else {
                        j -= 1;
                    }
                }
                Direction::Up => {
                    if i == 0 || matrix[i - 1][j] != -1 {
                        direction = Direction::Right;
                        j += 1
                    } else {
                        i -= 1;
                    }
                }
            }

            cur_head = cur.next.as_ref();
        }

        matrix
    }
}

#[cfg(test)]
mod tests {}
