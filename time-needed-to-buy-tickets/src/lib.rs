pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
    let mut q = std::collections::VecDeque::new();
    for (i, &ticket) in tickets.iter().enumerate() {
        q.push_back((i, ticket));
    }

    let mut count = 0;
    loop {
        println!("q={:?}", q);
        let (i, mut cur) = q.pop_front().unwrap();
        if i == k as usize && cur == 1 {
            count += 1;
            break;
        }
        if cur == 0 {
            q.push_back((i, cur));
            continue;
        }
        cur -= 1;
        count += 1;
        q.push_back((i, cur));
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // Explanation:
    // - In the first pass, everyone in the line buys a ticket and the line becomes [1, 2, 1].
    // - In the second pass, everyone in the line buys a ticket and the line becomes [0, 1, 0].
    // The person at position 2 has successfully bought 2 tickets and it took 3 + 3 = 6 seconds.

    #[test]
    fn it_works1() {
        let tickets = vec![2, 3, 2];
        let k = 2;
        let result = time_required_to_buy(tickets, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_works2() {
        let tickets = vec![5, 1, 1, 1];
        let k = 0;
        let result = time_required_to_buy(tickets, k);
        assert_eq!(result, 8);
    }
}
