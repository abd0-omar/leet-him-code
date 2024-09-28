struct MyCircularDeque {
    vec: Vec<Option<i32>>,
    front: usize,
    rear: usize,
    len: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let size = k as usize;
        Self {
            vec: vec![None; size],
            front: 0,
            rear: size - 1,
            len: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        // move front backward
        self.front = (self.front + self.vec.len() - 1) % self.vec.len();
        self.vec[self.front] = Some(value);
        self.len += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        // move rear forward
        self.rear = (self.rear + 1) % self.vec.len();
        self.vec[self.rear] = Some(value);
        self.len += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.vec[self.front] = None;
        self.front = (self.front + 1) % self.vec.len();
        self.len -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.vec[self.rear] = None;
        self.rear = (self.rear + self.vec.len() - 1) % self.vec.len();
        self.len -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1; // Return -1 if the deque is empty
        }
        self.vec[self.front].unwrap_or(-1)
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.vec[self.rear].unwrap_or(-1)
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.vec.len()
    }
}
