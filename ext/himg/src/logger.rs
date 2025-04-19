use std::time::Instant;

pub trait Logger {
    fn log(&mut self, message: &str);
}

pub struct NullLogger;

impl Logger for NullLogger {
    fn log(&mut self, _message: &str) {
        // no-op
    }
}

pub struct TimedLogger {
    initial_time: Instant,
    last_time: Instant,
}

impl TimedLogger {
    pub fn init() -> Self {
        let now = Instant::now();
        Self {
            initial_time: now,
            last_time: now,
        }
    }

    pub fn log_total_time(&mut self, message: &str) {
        let now = Instant::now();
        let diff = (now - self.initial_time).as_millis();
        println!("{message} in {diff}ms");

        self.last_time = now;
    }
}

impl Logger for TimedLogger {
    fn log(&mut self, message: &str) {
        let now = Instant::now();
        let diff = (now - self.last_time).as_millis();
        println!("{message} in {diff}ms");
        self.last_time = now;
    }
}
