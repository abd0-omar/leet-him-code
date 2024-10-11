use std::{cmp::Reverse, collections::BinaryHeap};

pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
    let mut events: Vec<_> = times
        .iter()
        .enumerate()
        .flat_map(|(idx, vec)| vec![(vec[0], true, idx), (vec[1], false, idx)])
        .collect();

    // dbg!(&events);

    events.sort_unstable();

    // dbg!(&events);

    let mut available_seats = (0..times.len()).map(Reverse).collect::<BinaryHeap<_>>();
    // dbg!(&available_seats);
    let mut occupied_seats = vec![None; times.len()];
    // dbg!(&occupied_seats);

    for (_time, is_arrival, idx) in events {
        if is_arrival {
            let seat = available_seats.pop().unwrap().0;
            occupied_seats[idx] = Some(seat);
            if idx as i32 == target_friend {
                return seat as i32;
            }
        } else {
            let seat = occupied_seats[idx].unwrap();
            available_seats.push(Reverse(seat));
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let times = vec![vec![1, 4], vec![2, 3], vec![4, 6]];
        let target_friend = 1;
        let output = 1;
        let result = smallest_chair(times, target_friend);
        assert_eq!(result, output);
    }
}
