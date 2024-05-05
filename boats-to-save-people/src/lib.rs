pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();
    let n = people.len();
    let mut st = 0;
    let mut end = n - 1;
    let mut initial_st = 0;

    let mut boats = 0;
    loop {
        println!("st={:?}", st);
        println!("end={:?}", end);
        if st == end {
            break;
        }
        if people[end] == -1 {
            end -= 1;
            continue;
        }
        let weight = people[st] + people[end];
        if people[st] != -1 && people[end] != -1 && weight <= limit {
            people[st] = -1;
            people[end] = -1;
            initial_st += 1;
            st = initial_st;
            boats += 1;
            println!("happened");
            continue;
        }
        dbg!(people[st] + people[end]);
        dbg!(people[st]);
        dbg!(people[end]);
        println!("weight={:?}", weight);
        if weight > limit {
            end -= 1;
            continue;
        }
        if weight < limit {
            st += 1;
            continue;
        }
    }

    println!("people={:?}", people);
    for person in people {
        if person != -1 {
            boats += 1;
        }
    }

    boats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let people = vec![3, 2, 2, 1];
        let limit = 3;
        let output = 3;
        // Explanation: 3 boats (1, 2), (2) and (3)
        let result = num_rescue_boats(people, limit);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works1() {
        let people = vec![1, 2];
        let limit = 3;
        let output = 1;
        // Explanation: 3 boats (1, 2), (2) and (3)
        let result = num_rescue_boats(people, limit);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works2() {
        let people = vec![3, 5, 3, 4];
        let limit = 5;
        let output = 4;
        // Explanation: 3 boats (1, 2), (2) and (3)
        let result = num_rescue_boats(people, limit);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works3() {
        let people = vec![5, 1, 4, 2];
        let limit = 6;
        let output = 2;
        // [1, 2, 4, 5]
        let result = num_rescue_boats(people, limit);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works4() {
        let people = vec![2, 2];
        let limit = 6;
        let output = 1;
        // [1, 2, 4, 5]
        let result = num_rescue_boats(people, limit);
        assert_eq!(result, output);
    }
}
