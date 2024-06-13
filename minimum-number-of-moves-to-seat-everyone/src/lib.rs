pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();
    let mut diff = 0;
    let n = seats.len();
    for i in 0..n {
        diff += (seats[i] - students[i]).abs();
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let seats = vec![3, 1, 5];
        let students = vec![2, 7, 4];
        let output = 4;
        let result = min_moves_to_seat(seats, students);
        assert_eq!(result, output);
    }
}
