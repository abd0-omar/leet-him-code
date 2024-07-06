pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let mut count = 1;
    let mut rotate = false;
    for i in 0..time {
        if count == n || (count == 1 && i != 0) {
            rotate = !rotate;
        }
        if !rotate {
            count += 1;
        } else {
            count -= 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 4;
        let time = 5;
        let output = 2;
        // Explanation: People pass the pillow in the following way: 1 -> 2 -> 3 -> 4 -> 3 -> 2.
        let result = pass_the_pillow(n, time);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 3;
        let time = 2;
        let output = 3;
        let result = pass_the_pillow(n, time);
        assert_eq!(result, output);
    }
}
