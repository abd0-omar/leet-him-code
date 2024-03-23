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

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let vec = list_to_vec(head);
    println!("vec={:?}", vec);

    vec == vec.iter().rev().cloned().collect::<Vec<i32>>()
}

fn list_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut rezult = Vec::new();

    while let Some(cur) = head.take() {
        rezult.push(cur.as_ref().val);
        head = cur.next;
    }

    rezult
}
