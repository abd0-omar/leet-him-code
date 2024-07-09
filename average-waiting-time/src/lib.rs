pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut total_wait = 0f64;
    let mut chef_idle_time = 0f64;

    for customer in customers.iter() {
        let arrival = customer[0] as f64;
        let time = customer[1] as f64;

        if chef_idle_time <= arrival {
            total_wait += time;
            chef_idle_time = arrival + time;
        } else {
            total_wait += chef_idle_time - arrival + time;
            chef_idle_time += time;
        }
    }

    total_wait / customers.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let customers = vec![vec![1, 2], vec![2, 5], vec![4, 3]];
        let output = 5.00000;
        let result = average_waiting_time(customers);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let customers = vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]];
        let output = 3.25000;
        let result = average_waiting_time(customers);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let customers = vec![
            vec![2, 3],
            vec![6, 3],
            vec![7, 5],
            vec![11, 3],
            vec![15, 2],
            vec![18, 1],
        ];
        let output = 4.16667;
        let result = average_waiting_time(customers);
        assert_eq!(result, output);
    }
}
