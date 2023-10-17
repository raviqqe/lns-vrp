#[derive(Clone, Copy, Debug)]
pub struct Route {
    visited_stops: usize,
    current_stop: usize,
}

impl Route {
    pub fn new(visited_stops: usize, current_stop: usize) -> Self {
        Self {
            visited_stops,
            current_stop,
        }
    }

    pub fn visited_stops(self) -> usize {
        self.visited_stops
    }

    pub fn current_stop(self) -> usize {
        self.current_stop
    }
}
