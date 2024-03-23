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

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = head.clone();
    let mut fast = head.clone();
    while fast.is_some() || fast.clone().unwrap().next.is_none() {
        slow = slow.unwrap().next;
        fast = match fast.unwrap().next {
            Some(f) => f.next,
            None => None,
        }
    }
    slow
}
