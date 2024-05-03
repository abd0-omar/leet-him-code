pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();
    let mut monotonic_stack = vec![-1; n];
    let mut stack = Vec::with_capacity(n);
    for (i, h) in height.iter().enumerate() {
        if stack.is_empty() {
            stack.push((i, h));
        } else {
            let top = stack.last().unwrap().1;
            if top < h {
                while !stack.is_empty() {
                    if stack.last().unwrap().1 >= &h {
                        break;
                    }
                    let popped_i = stack.pop().unwrap().0;
                    monotonic_stack[popped_i] = i as i32;
                }
            }
            stack.push((i, h));
        }
    }

    println!("heights={:?}", height);
    println!("monotonic_stack={:?}", monotonic_stack);

    let mut trapped = Vec::new();
    for i in 0..n {
        trapped.push(monotonic_stack[i] - height[i] + 1);
    }

    dbg!(trapped);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let output = 6;
        // Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
        let result = trap(height);
        assert_eq!(result, output);
    }
    // Example 2:
    //
    // Input: height = [4,2,0,3,2,5]
    // Output: 9
}
