/**
 * # IsDone trait
 *
 * many of the actions require a loop/time threshold
 * the logic is usually implemented as `while not_done()`
 * this module holds 2 of those implementations.
 * 1. for_loop (allows n loops before is_done)
 * 2. time_threshold_ms (allows n ms of runtime before is_done)
 *
 * the public facing API of these modules will be
 * dyn IsDone
 *
 * ## Counter
 *
 * basically a for loop that allow n iterations
 *
 * ### example
 *
 * ```
 *
 * ```
 *
 * ## TimeKeeper

 * keep track of time, to timeout actions after threshold
 *
 * ### example
 *
 * ```
 *
 * ```
 *
*/
use std::time::{Duration, Instant};

pub trait IsDone {
    fn increment_and_check(&mut self) -> bool;
    fn check(&self) -> bool;
}

pub struct TimeKeeper {
    start_time: Instant,
    time_threshold_ms: Duration,
}

impl TimeKeeper {
    pub fn new(time_threshold_ms: u64) -> Self {
        TimeKeeper {
            start_time: Instant::now(),
            time_threshold_ms: Duration::from_millis(time_threshold_ms),
        }
    }

    pub fn is_over(&self) -> bool {
        self.start_time.elapsed() > self.time_threshold_ms
    }
}

impl IsDone for TimeKeeper {
    fn increment_and_check(&mut self) -> bool {
        self.is_over()
    }
    fn check(&self) -> bool {
        self.is_over()
    }
}

pub struct Counter {
    depth: usize,
    max_depth: usize,
}

impl Counter {
    pub fn new(max_depth: usize) -> Self {
        Counter {
            depth: 0,
            max_depth,
        }
    }
}

impl IsDone for Counter {
    fn increment_and_check(&mut self) -> bool {
        self.depth += 1;
        self.depth > self.max_depth
    }

    fn check(&self) -> bool {
        self.depth > self.max_depth
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_time_keeper_is_done() {
        let mut time_keeper = TimeKeeper::new(0);
        assert_eq!(time_keeper.check(), true);
        assert_eq!(time_keeper.increment_and_check(), true);

        let time_keeper = TimeKeeper::new(1);
        assert_eq!(time_keeper.check(), false);
        thread::sleep(Duration::from_millis(1));
        assert_eq!(time_keeper.check(), true);
    }

    #[test]
    fn test_counter_is_done() {
        let mut counter = Counter::new(0);
        assert_eq!(counter.check(), false);
        assert_eq!(counter.increment_and_check(), true);

        let mut counter = Counter::new(1);
        assert_eq!(counter.increment_and_check(), false);
        assert_eq!(counter.increment_and_check(), true);
        assert_eq!(counter.check(), true);

        let mut counter = Counter::new(1);
        assert_eq!(counter.increment_and_check(), false);
        assert_eq!(counter.check(), false);
        assert_eq!(counter.increment_and_check(), true);
        assert_eq!(counter.check(), true);
    }

    #[test]
    fn test_is_time_over() {
        let time_keeper = TimeKeeper::new(0);
        assert!(time_keeper.is_over());
    }
}
