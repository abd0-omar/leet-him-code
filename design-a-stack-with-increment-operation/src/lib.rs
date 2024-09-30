#[derive(Debug)]
struct CustomStack {
    stack: Vec<i32>,
    len: usize,
    capcity: usize,
}

impl CustomStack {
    fn new(maxSize: i32) -> Self {
        let max_size = maxSize as usize;
        Self {
            stack: Vec::with_capacity(max_size),
            len: 0,
            capcity: max_size,
        }
    }

    fn push(&mut self, x: i32) {
        if self.len == self.capcity - 1 {
            return;
        }
        self.stack.push(x);
        self.len += 1;
    }

    fn pop(&mut self) -> i32 {
        if let Some(popped) = self.stack.pop() {
            self.len -= 1;
            popped
        } else {
            -1
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        println!("{:?}", self);
        for i in 0..((k as usize).min(self.len)) {
            self.stack[i] += val;
        }
    }
}
