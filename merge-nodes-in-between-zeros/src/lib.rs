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

// 0 3 1 0 4 5 2 0

// h n

//   h n
//     h n
// if n is a zero, create a node and put in it the sum, 4

//

pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    let mut sum = 0;
    while let Some(mut cur) = head.take() {
        head = cur.next.take();
        sum += cur.val;

        if let Some(h) = head.as_ref() {
            if h.val == 0 {
                tail.next = Some(Box::new(ListNode::new(sum)));
                tail = tail.next.as_mut().unwrap();
            }
        }
    }

    dummy.next
}

#[cfg(test)]
mod tests {}
