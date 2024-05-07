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
pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut reversed_head = reverse_ll(head.clone());
    let mut prev_remainder = 0;
    let mut curr = &mut reversed_head;

    while let Some(cur_node) = curr {
        let multiplication = cur_node.val * 2;
        let multiplication_first_digit = multiplication % 10;
        cur_node.val = multiplication_first_digit + prev_remainder;
        prev_remainder = multiplication / 10;

        curr = &mut cur_node.next;
    }
    if prev_remainder > 0 {
        // curr is now at None "end of the list"
        *curr = Some(Box::new(ListNode::new(prev_remainder)));
    }
    reverse_ll(reversed_head)
}

fn reverse_ll(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;

    while let Some(mut cur_node) = head {
        let next = cur_node.next.take();
        cur_node.next = prev;
        prev = Some(cur_node);
        head = next;
    }
    prev
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
