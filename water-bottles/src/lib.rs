pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    // 15      c:15
    // 15/4 -> 3.75    c: 15 + 3 -> 18
    // 4/(3.75 aka 4) -> 1 c: 19
    // ignore the comment above
    let mut bottles_drank = 0;
    while num_bottles >= num_exchange {
        let division = num_bottles / num_exchange;

        bottles_drank += num_exchange * division;
        num_bottles -= num_exchange * division;

        num_bottles += division;
    }
    bottles_drank + num_bottles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let num_bottles = 15;
        let num_exchange = 4;
        let output = 19;
        let result = num_water_bottles(num_bottles, num_exchange);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let num_bottles = 9;
        let num_exchange = 3;
        let output = 13;
        let result = num_water_bottles(num_bottles, num_exchange);
        assert_eq!(result, output);
    }
}
