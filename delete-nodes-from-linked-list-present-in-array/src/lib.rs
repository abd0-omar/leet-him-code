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

pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let hs: std::collections::HashSet<i32> = std::collections::HashSet::from_iter(nums.into_iter());

    dbg!(&hs);

    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut current = &mut dummy;

    while let Some(next) = current.next.take() {
        if hs.contains(&next.val) {
            // ignore cur next
            current.next = next.next;
        } else {
            current.next = Some(next); // Re-attach the node, because we did take()
            current = current.next.as_mut().unwrap(); // move to the next
        }
        dbg!(&current);
    }

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3];
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));
        let output = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(5))),
        }));
        let result = modified_list(nums, head);
        assert_eq!(result, output);
    }
}
