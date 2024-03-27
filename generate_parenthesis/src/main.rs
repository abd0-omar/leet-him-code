fn main() {
    let n = 3;
    let mut res: Vec<String> = Vec::new();
    let mut stack: Vec<char> = vec![];
    fn recursion(n: i32, o: i32, c: i32, res: &mut Vec<String>, stack: &mut Vec<char>) {
        if c == n && o == n {
            res.push(stack.iter().collect());
            return;
        }

        if o < n {
            stack.push('(');
            recursion(n, o + 1, c, res, stack);
            stack.pop();
        }

        if c < o {
            stack.push(')');
            recursion(n, o, c + 1, res, stack);
            stack.pop();
        }
    }

    recursion(n, 0, 0, &mut res, &mut stack);

    println!("{:?}", res);
}
