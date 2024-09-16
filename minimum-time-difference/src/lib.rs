pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    // convert to minutes
    // sort
    // circular  mod with comparing the first one with the last one via getting the -ve  value then
    // increasing it by a full rotation then modding it, would become more clear once we written
    // this only applies to the first and last time points

    let n = time_points.len();
    let mut minutes = Vec::with_capacity(n);
    for time in time_points {
        let hour = &time[..2].parse::<i32>().unwrap() * 60;

        // avoid the ':'

        let minute = &time[3..].parse::<i32>().unwrap();
        minutes.push(hour + minute);
    }
    // max minutes: 1440

    minutes.sort_unstable();

    let mut min = i32::MAX;

    for i in 1..n {
        min = min.min(minutes[i] - minutes[i - 1]);
    }

    // compare first with last

    let first = minutes[0];
    let last = minutes[n - 1];
    // could be negative, so increase it with max
    let calc = ((first - last) + 1440) % 1440;

    min.min(calc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let time_points = vec!["23:59".to_string(), "00:00".to_string()];
        let output = 1;
        let result = find_min_difference(time_points);
        assert_eq!(result, output);
    }
}
