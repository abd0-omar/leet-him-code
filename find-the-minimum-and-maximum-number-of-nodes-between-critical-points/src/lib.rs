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
pub fn nodes_between_critical_points(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![-1, -1];
    let mut min_distance = i32::MAX;
    let mut max_distance = 0;

    let mut first_critical_point = None;
    let mut prev_critical_point = None;

    let mut prv: Option<i32> = None;
    let mut count = 0;

    while let Some(mut cur) = head.take() {
        let cur_val = cur.val;
        let nxt = cur.next.take();

        if let Some(p) = prv {
            if let Some(n) = nxt.as_deref() {
                // local maxima
                if cur_val > p && cur_val > n.val {
                    if let Some(p_crit) = prev_critical_point {
                        if count - p_crit < min_distance {
                            min_distance = count - p_crit;
                            result[0] = min_distance;
                        }
                    }

                    if let Some(f_crit) = first_critical_point {
                        if count - f_crit > max_distance {
                            max_distance = count - f_crit;
                            result[1] = max_distance;
                        }
                    } else {
                        first_critical_point = Some(count);
                    }
                    prev_critical_point = Some(count);
                }
                // local minima
                if cur_val < p && cur_val < n.val {
                    if let Some(p_crit) = prev_critical_point {
                        if count - p_crit < min_distance {
                            min_distance = count - p_crit;
                            result[0] = min_distance;
                        }
                    }
                    if let Some(f_crit) = first_critical_point {
                        if count - f_crit > max_distance {
                            max_distance = count - f_crit;
                            result[1] = max_distance;
                        }
                    } else {
                        first_critical_point = Some(count);
                    }
                    prev_critical_point = Some(count);
                }
            }
        }

        prv = Some(cur_val);
        head = nxt;
        count += 1;
    }

    result
}

#[cfg(test)]
mod tests {}
