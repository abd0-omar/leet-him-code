fn main() {
    println!("Hello, world!");
}

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
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if let Some(list_one) = l1 {
        let mut head = list_one.clone();
        let mut str_num = String::from(char::from_digit(head.val as u32, 10).unwrap());
        while let Some(cur) = head.next.clone() {
            str_num.push(char::from_digit(cur.val as u32, 10).unwrap());
            head = head.next.unwrap();
        }
        println!("str_num={:?}", str_num);
    }

    todo!()
}
