// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_result = Box::new(ListNode::new(0));
        let mut tail = &mut dummy_result;

        while let Some(cur) = head.take() {
            let nxt = cur.next;

            tail.next = Some(Box::new(ListNode::new(cur.val)));
            // move the tail
            tail = tail.next.as_mut().unwrap();

            if let Some(n) = nxt.clone() {
                let gcd = euclidean_gcd(cur.val, n.val);
                tail.next = Some(Box::new(ListNode {
                    val: gcd,
                    next: Some(Box::new(ListNode::new(n.val))),
                }));
                // move the tail
                tail = tail.next.as_mut().unwrap();
            }

            head = nxt;
        }

        dummy_result.next
    }
}

fn euclidean_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// non-recursive
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a > 0 && b > 0 {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }

    if a == 0 {
        b
    } else {
        a
    }
}

fn gcd_recursive(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }

    gcd_recursive(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
