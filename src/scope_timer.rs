use log::info;
use std::time::Instant;

#[derive(Debug)]
pub struct ScopeTimer {
    name: String,
    start: Instant,
}

impl ScopeTimer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            start: Instant::now(),
        }
    }
}

impl Drop for ScopeTimer {
    fn drop(&mut self) {
        info!("[{}]: {:?} elapsed time", self.name, self.start.elapsed());
    }
}
