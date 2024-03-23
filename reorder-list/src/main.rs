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
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        let mut length = 0;
        let mut list = head.as_mut();
        // find the total length of the list
        // Using the slow & fast pointer is probably not possible
        while let Some(node) = list {
            length += 1;
            list = node.next.as_mut();
        }
        // if length is less than or equal to 2 then exit the function
        if length <= 2 {
            return;
        }
        // find the middle node to split the list
        let mut mid = head.as_mut();
        for _ in 0..length / 2 {
            mid = match mid {
                None => unreachable!("Traversing half of the list. Cannot reach the end."),
                Some(node) => node.next.as_mut(),
            }
        }
        println!("mid={:?}", mid);
        // Tail is the right half of the list.
        // Here we use take() to break the linked list
        // so the end of the first half now points to None
        // and tail holds the second half of the linked list
        let mut tail = mid.expect("Mid cannot be None.").as_mut().next.take();
        println!("tail={:?}", tail);
        // reverse the tail
        let mut reversed: Option<Box<ListNode>> = None;
        while let Some(mut node) = tail {
            tail = node.next;
            node.next = reversed;
            reversed = Some(node);
        }
        println!("reversed={:?}", reversed);
        // merge the first half(head) & the second reversed half(reversed)
        // This solution to merge two linked lists is clever
        // https://leetcode.com/problems/merge-two-sorted-lists/solutions/2947855/simple-and-efficient-rust-8-liner/
        // mover is the variable that traverses and modifies "head"
        let mut mover: &mut _ = &mut reversed;
        while head.is_some() {
            std::mem::swap(mover, &mut head);
            mover = &mut mover.as_mut().unwrap().next;
        }
        std::mem::swap(head, &mut reversed);
    }
}

fn main() {
    // Example 1
    let mut head1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode::new(4))),
            })),
        })),
    }));
    Solution::reorder_list(&mut head1);
    println!("{:?}", head1);

    // Example 2
    let mut head2 = Some(Box::new(ListNode {
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
    Solution::reorder_list(&mut head2);
    println!("{:?}", head2);
}
