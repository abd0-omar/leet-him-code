#[derive(Ord, Eq, PartialEq, PartialOrd, Debug)]
struct Event {
    st: i32,
    end: i32,
}

impl Event {
    fn new(st: i32, end: i32) -> Self {
        Self { st, end }
    }
}

// sort the events for easier lookup using binary search
struct MyCalendar {
    // end not included
    event: Vec<Event>,
    // could be done as a unit struct
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self { event: Vec::new() }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut l = 0;
        let mut r = self.event.len();

        while l < r {
            let mid = l + (r - l) / 2;

            if self.event[mid].end <= start {
                l = mid + 1;
            } else if self.event[mid].st >= end {
                r = mid;
            } else {
                return false;
            }
        }
        self.event.push(Event::new(start, end));
        self.event.sort_unstable();
        println!("{:?}", self.event);
        true
    }
}
