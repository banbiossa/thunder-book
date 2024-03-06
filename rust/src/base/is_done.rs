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
 Box<dyn FnMut() -> bool>
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

pub type Stopper = Box<dyn FnMut() -> bool>;

// depth based stopping condition
// you can call this max_depth times before it returns true
// basically just a while loop
pub fn depth_stopper(max_depth: usize) -> Box<dyn FnMut() -> bool> {
    let mut depth = 0;
    Box::new(move || {
        depth += 1;
        depth > max_depth
    })
}

// time_keeper basesd stopping condition
pub fn time_stopper(time_threshold_ms: u64) -> Box<dyn FnMut() -> bool> {
    //
    let mut time_keeper = TimeKeeper::new(time_threshold_ms);
    Box::new(move || time_keeper.is_over())
}

// no stopping (but necessary as placeholder)
pub fn no_stop() -> Box<dyn FnMut() -> bool> {
    Box::new(move || false)
}

struct TimeKeeper {
    start_time: Option<Instant>,
    time_threshold_ms: Duration,
}

impl TimeKeeper {
    pub fn new(time_threshold_ms: u64) -> Self {
        TimeKeeper {
            start_time: None,
            time_threshold_ms: Duration::from_millis(time_threshold_ms),
        }
    }

    // let start_time be the first call to is_over
    pub fn is_over(&mut self) -> bool {
        match self.start_time {
            Some(start_time) => start_time.elapsed() > self.time_threshold_ms,
            None => {
                self.start_time = Some(Instant::now());
                self.start_time.unwrap().elapsed() > self.time_threshold_ms
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread;

    #[test]
    fn test_no_stop() {
        let mut stopper = no_stop();
        assert_eq!(stopper(), false);
        assert_eq!(stopper(), false);
        assert_eq!(stopper(), false);
    }

    #[test]
    fn test_time_stopper() {
        let mut stopper = time_stopper(0);
        assert_eq!(stopper(), true);

        // this test may be flaky, as it is based on running time
        let mut stopper = time_stopper(10);
        assert_eq!(stopper(), false);
        thread::sleep(Duration::from_millis(10));
        assert_eq!(stopper(), true);
    }

    #[test]
    fn test_depth_stopper() {
        // test that the stopper logic works
        let mut stopper = depth_stopper(0);
        // 0st call is true
        assert_eq!(stopper(), true);
        // 1st call is also true
        assert_eq!(stopper(), true);

        let mut stopper = depth_stopper(2);
        assert_eq!(stopper(), false);
        assert_eq!(stopper(), false);
        assert_eq!(stopper(), true);
    }

    #[test]
    fn test_is_time_over() {
        let mut time_keeper = TimeKeeper::new(0);
        assert_eq!(time_keeper.is_over(), true);
    }
}
