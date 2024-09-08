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
#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut result = Vec::with_capacity(k as usize);

        // calculate the list len
        let mut new_head = head.as_ref();

        let mut len = 0;
        while let Some(cur_node) = new_head {
            len += 1;

            new_head = cur_node.next.as_ref();
        }

        // every part has len / k elements
        // first len % k elements have an extra element
        // for our example
        // len / k -> 0
        // len % k -> 3
        // all will have 0 elemtns, except for the first three would have 0 + 1 more

        let part_size = len / k;
        let extra_parts = len % k;

        for i in 0..k {
            let mut part_head = head.take();
            let mut part_tail = &mut part_head;

            // the part with the extra `1` or not
            let current_part_size = part_size + if i < extra_parts { 1 } else { 0 };

            for _ in 0..current_part_size - 1 {
                if let Some(node) = part_tail {
                    part_tail = &mut node.next;
                }
            }

            if let Some(node) = part_tail {
                // disconnect the rest of the list
                head = node.next.take();
            }
            // dbg!(&part_head);

            result.push(part_head);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        }));

        let k = 5;
        let output = vec![
            Some(Box::new(ListNode::new(1))),
            Some(Box::new(ListNode::new(2))),
            Some(Box::new(ListNode::new(3))),
            None,
            None,
        ];

        let result = Solution::split_list_to_parts(head, k);

        assert_eq!(result, output);
    }
}
