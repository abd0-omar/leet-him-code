fn main() {
    println!("Hello, world!");
}

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

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut prev = None;
    let mut cur = head.clone();

    while let Some(mut cur_node) = cur {
        let next_node = cur_node.next;
        cur_node.next = prev;
        prev = Some(cur_node);
        cur = next_node;
    }

    prev // Return the new head of the reversed list
}
