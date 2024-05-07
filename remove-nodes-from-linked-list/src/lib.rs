#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = Vec::new();
    let mut cur_head = head;
    while let Some(node) = cur_head {
        while !stack.is_empty() && *stack.last().unwrap() < node.val {
            stack.pop();
        }
        stack.push(node.val);
        cur_head = node.next;
    }

    let mut head = None;
    while let Some(val) = stack.pop() {
        head = Some(Box::new(ListNode { val, next: head }));
    }
    head
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {}
}
