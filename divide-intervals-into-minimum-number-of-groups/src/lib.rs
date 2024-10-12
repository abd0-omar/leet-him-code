pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    /*
    [[5,10],[6,8],[1,5],[2,3],[1,10]]
    sort
    [1, 5], [1, 10], [2, 3], [5, 10], [6, 8]

    [1, 5], [1, 10], [2, 3], [5, 10], [6, 8]
    */
    // let mut events: Vec<_> = intervals
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(idx, vec)| vec![(vec[0], true, idx), (vec[1], false, idx)])
    //     .collect();
    // events.sort_unstable();
    // dbg!(&events);
    let (mut start, mut end) = (vec![], vec![]);

    for vec in intervals.iter() {
        start.push(vec[0]);
        end.push(vec[1]);
    }

    start.sort_unstable();
    end.sort_unstable();

    let (mut i, mut j) = (0, 0);
    let mut groups = 0;
    let mut res = 0;

    while i < intervals.len() {
        if start[i] <= end[j] {
            i += 1;
            groups += 1;
        } else {
            groups -= 1;
            j += 1;
        }
        res = res.max(groups);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let intervals = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
        let output = 3;
        let result = min_groups(intervals);
        assert_eq!(result, output);
    }
}
