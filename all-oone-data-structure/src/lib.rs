use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct AllOne {
    hm: HashMap<String, i32>,
    max_heap: BinaryHeap<(i32, String)>,
    min_heap: BinaryHeap<(Reverse<i32>, String)>,
}

impl AllOne {
    fn new() -> Self {
        Self {
            hm: HashMap::new(),
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        let count = self.hm.entry(key.clone()).or_insert(0);
        *count += 1;
        self.max_heap.push((*count, key.clone()));
        self.min_heap.push((Reverse(*count), key));
    }

    fn dec(&mut self, key: String) {
        if let Some(count) = self.hm.get(&key) {
            if *count == 1 {
                self.hm.remove(&key);
            } else {
                self.max_heap.push((*count - 1, key.clone()));
                self.min_heap.push((Reverse(*count - 1), key.clone()));
                *self.hm.get_mut(&key).unwrap() -= 1;
            }
        }
    }

    fn get_max_key(&mut self) -> String {
        // it is possible that it got remove from hm but not from heap
        while let Some((heap_count, key)) = self.max_heap.peek() {
            if let Some(hm_count) = self.hm.get(key) {
                if heap_count == hm_count {
                    return key.clone();
                }
                self.max_heap.pop();
            } else {
                self.max_heap.pop();
            }
        }
        String::new()
    }

    fn get_min_key(&mut self) -> String {
        while let Some((Reverse(heap_count), key)) = self.min_heap.peek() {
            if let Some(hm_count) = self.hm.get(key) {
                if heap_count == hm_count {
                    return key.clone();
                }
                self.min_heap.pop();
            } else {
                self.min_heap.pop();
            }
        }
        String::new()
    }
}
