struct MyCalendarTwo {
    events: Vec<(i32, i32)>,
    overlaps: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            overlaps: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(overlap_start, overlap_end) in &self.overlaps {
            if start < overlap_end && end > overlap_start {
                return false;
            }
        }

        for &(events_start, events_end) in &self.events {
            if start < events_end && end > events_start {
                let overlap_start = start.max(events_start);
                let overlap_end = end.min(events_end);
                self.overlaps.push((overlap_start, overlap_end));
            }
        }

        self.events.push((start, end));
        true
    }
}
