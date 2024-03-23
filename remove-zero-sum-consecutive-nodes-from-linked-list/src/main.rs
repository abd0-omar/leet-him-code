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

fn vec_to_linked_list(mut nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    // let mut tail = &mut head;

    while let Some(num) = nums.pop() {
        if num != 0 {
            let mut node = Box::new(ListNode::new(num));
            node.next = head;
            head = Some(node);
        }
    }

    head
}

pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vec = vec![];
    // let mut sum_vec = vec![0];
    let mut hs = std::collections::HashSet::from([0]);
    let mut sum = 0;
    while let Some(cur) = head {
        vec.push(cur.val);
        sum += cur.val;
        // sum_vec.push(cur.val + sum_vec.last().unwrap());
        // println!("sum_vec={:?}", sum_vec);
        if !hs.insert(sum) {
            let stoppage = sum;
            while let Some(x) = vec.pop() {
                sum -= x;
                hs.remove(&sum);
                if sum == stoppage {
                    break;
                }
            }
        }
        head = cur.next;
    }
    println!("{:?}", vec);
    vec_to_linked_list(vec.into_iter().rev().collect())
}
