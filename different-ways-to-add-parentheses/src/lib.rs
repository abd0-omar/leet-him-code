// pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
//     // it looks like backtracking and the input is low too
//     // but will try out the parsing first
//     //           2*3-4*5
//     //  take the first two expressions or leave
//     //              take                  leave
//     //             6-4*5                   2*3-4*5
//     //      2*5        6-4*5            2*(-1)*5                 2*3-4*5
//     //  10   2*5     6-20             -2*5      2*-1*5      2*3-20       2*3-4*5
//     // something like that, I think that's the idea
//
//     // testing Expression struct
//     /*
//         let mut new_expression = Expression::from(expression);
//         dbg!(&new_expression);
//
//         let result = new_expression.calculate_expression_at_cur_idx();
//         dbg!(&result);
//         dbg!(&new_expression);
//
//         dbg!("second time");
//
//         let result = new_expression.calculate_expression_at_cur_idx();
//         dbg!(&result);
//         dbg!(&new_expression);
//
//         dbg!("third time");
//         let result = new_expression.calculate_expression_at_cur_idx();
//         dbg!(&result);
//         dbg!(&new_expression);
//
//         dbg!("fourth time");
//         let result = new_expression.calculate_expression_at_cur_idx();
//         dbg!(&result);
//         dbg!(&new_expression);
//     */
//
//     let mut result = Vec::new();
//     _diff_ways_to_compute(expression.as_bytes(), 0, &mut result, 0);
//     result
// }
//
// #[derive(Debug)]
// struct Expression {
//     value: Vec<u8>,
//     idx: usize,
// }
//
// impl Expression {
//     fn calculate_expression_at_cur_idx(&mut self) -> Result<i32, String> {
//         // idx will move to the first number after * - +
//         // calculate expression at cur_idx
//         // first_number could be up to two digits
//         // let first_number = self.value[self.idx];
//         let st = self.idx;
//         let mut end = self.idx;
//
//         while end < self.value.len() && self.value[end].is_ascii_digit() {
//             end += 1;
//         }
//
//         // end would "end" up at * + - or at the end, ignore the end case for now
//         let first_number = String::from_utf8(self.value[st..end].to_vec())
//             .unwrap()
//             .parse::<i32>()
//             .unwrap();
//         dbg!(&first_number);
//
//         let sign_pos = end;
//         if end >= self.value.len() {
//             return Err("sign can not be a number, end len case reached".to_string());
//         }
//         let sign = match self.value[end] {
//             b'+' | b'-' | b'*' => self.value[end],
//             _ => return Err("sign can not be a number, end len case reached".to_string()),
//         };
//         dbg!(&char::from_u32(sign as u32).unwrap());
//
//         // parse second number
//         let st = if end + 1 >= self.value.len() {
//             return Err("st exceeds len in second number".to_string());
//         } else {
//             end + 1
//         };
//
//         let mut end = st;
//
//         while end < self.value.len() && self.value[end].is_ascii_digit() {
//             end += 1;
//         }
//
//         let second_number = String::from_utf8(self.value[st..end].to_vec())
//             .unwrap()
//             .parse::<i32>()
//             .unwrap();
//         dbg!(&second_number);
//
//         let result = match sign {
//             b'+' => first_number + second_number,
//             b'-' => first_number - second_number,
//             b'*' => first_number * second_number,
//             _ => unreachable!(),
//         };
//
//         // move idx to the next expression
//         self.idx = sign_pos + 1;
//
//         // rust is great at parsing, no rust is great at everything!, except hot reload, but who
//         // needs that, in that time fill your waiting with ast8far
//         Ok(result)
//     }
// }
//
// impl From<String> for Expression {
//     fn from(value: String) -> Self {
//         Self {
//             value: value.into_bytes(),
//             idx: 0,
//         }
//     }
// }
//
// impl From<&[u8]> for Expression {
//     fn from(value: &[u8]) -> Self {
//         Self {
//             value: value.to_vec(),
//             idx: 0,
//         }
//     }
// }
//
// pub fn _diff_ways_to_compute(
//     expression: &[u8],
//     idx: usize,
//     result: &mut Vec<i32>,
//     mut cur_result: i32,
// ) {
//     if idx >= expression.len() {
//         result.push(cur_result);
//         return;
//     }
//
//     let mut new_expression = Expression::from(expression);
//     new_expression.idx = idx;
//
//     // add cur_result somehow
//     let expression_value_at_cur_idx = match new_expression.calculate_expression_at_cur_idx() {
//         Ok(res) => res,
//         Err(_) => {
//             result.push(cur_result);
//             return;
//         }
//     };
//
//     cur_result += expression_value_at_cur_idx;
//     let _take = _diff_ways_to_compute(expression, new_expression.idx, result, cur_result);
//     cur_result -= expression_value_at_cur_idx;
//
//     cur_result += expression_value_at_cur_idx;
//     let _leave = _diff_ways_to_compute(expression, new_expression.idx, result, cur_result);
//
//     // update idx
//
//     // push
//     // then
//     // pop
//     // that's backtracking
//     // I guess I'm the backtracKING
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works0() {
//         let expression = "2-1-1".to_string();
//         let output = vec![0, 2];
//         let result = diff_ways_to_compute(expression);
//         assert_eq!(result, output);
//     }
//
//     #[test]
//     fn it_works1() {
//         let expression = "2*3-4*5".to_string();
//         let output = vec![-34, -14, -10, -10, 10];
//         let result = diff_ways_to_compute(expression);
//         assert_eq!(result, output);
//     }
// }

pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    let mut result = Vec::new();
    _diff_ways_to_compute(&Expression::from(expression), &mut result);
    result
}

#[derive(Debug, Clone)]
struct Expression {
    value: Vec<u8>,
}

impl Expression {
    fn calculate_expression(&self, start: usize, end: usize) -> i32 {
        let sub_expr = &self.value[start..end];
        let num = String::from_utf8(sub_expr.to_vec())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        num
    }
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Self {
            value: value.into_bytes(),
        }
    }
}

fn _diff_ways_to_compute(expr: &Expression, result: &mut Vec<i32>) {
    let mut found_operator = false;

    for i in 0..expr.value.len() {
        let ch = expr.value[i];

        // Check if current character is an operator
        if ch == b'+' || ch == b'-' || ch == b'*' {
            found_operator = true;

            // Recursively compute results for the left and right sides
            let mut left_results = Vec::new();
            let mut right_results = Vec::new();

            // empty slice
            let left_expr = Expression {
                value: expr.value[..i].to_vec(),
            };
            let right_expr = Expression {
                value: expr.value[i + 1..].to_vec(),
            };

            _diff_ways_to_compute(&left_expr, &mut left_results);
            _diff_ways_to_compute(&right_expr, &mut right_results);

            // Combine results from left and right using the current operator
            for left in left_results.iter() {
                for right in right_results.iter() {
                    let value = match ch {
                        b'+' => left + right,
                        b'-' => left - right,
                        b'*' => left * right,
                        _ => unreachable!(),
                    };
                    result.push(value);
                }
            }
        }
    }

    // Base case: if no operator is found, the entire expression is a single number
    if !found_operator {
        result.push(expr.calculate_expression(0, expr.value.len()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let expression = "2-1-1".to_string();
        let output = vec![0, 2];
        let result = diff_ways_to_compute(expression);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let expression = "2*3-4*5".to_string();
        let output = vec![-34, -14, -10, -10, 10];
        let result = diff_ways_to_compute(expression);
        assert_eq!(result, output);
    }
}
