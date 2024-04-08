pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let n = students.len();
    let mut mark_students = vec![false; n];

    let mut j = 0;
    'outer: for i in 0..n {
        let mut loop_count = 0;
        loop {
            if sandwiches[i] == students[j] && !mark_students[j] {
                mark_students[j] = true;
                break;
            }
            if loop_count == n {
                println!("loop_count={:?}", loop_count);
                break 'outer;
            }
            loop_count += 1;
            j += 1;
            j %= n;
        }
        println!("mark_students={:?}", mark_students);
    }

    mark_students.iter().filter(|&f| f == &false).count() as _
}

fn _shift_left(students: &mut Vec<i32>, n: usize) {
    let first = students[0];
    for i in 1..n {
        students[i - 1] = students[i];
    }
    students[n - 1] = first;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        //                  x  x  x  x
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        let result = count_students(students, sandwiches);
        assert_eq!(result, 0);
    }

    #[test]
    fn it_works2() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        let result = count_students(students, sandwiches);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works3() {
        let students = vec![1, 1];
        let sandwiches = vec![0, 1];
        let result = count_students(students, sandwiches);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works4() {
        let students = vec![0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1];
        let sandwiches = vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0];
        let result = count_students(students, sandwiches);
        assert_eq!(result, 5);
    }

    /*
    Input: students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]
    Output: 3
    [1, 1, 1, 0, 0, 1],   [1, 0, 0, 0, 1, 1]
    [1, 1, 0, 0, 1],   [0, 0, 0, 1, 1] // take
    [1, 1, 0, 0, 1],   [0, 0, 0, 1, 1]
    // 1: 3, 0: 2    /  0
    [1, 0, 0, 1, 1],   [0, 0, 0, 1, 1] // end of queue
    [0, 0, 1, 1, 1],   [0, 0, 0, 1, 1] // end of queue
    [0, 1, 1, 1],   [0, 0, 1, 1] // take
    [1, 1, 1],   [0, 1, 1] // take
    // 1: 3, 0:0  /  0
    [1, 1, 1],   [0, 1, 1] // take
    */
}
