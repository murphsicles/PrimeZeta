/// Stub scheduler type for v0.5.0 compilation
#[derive(Debug)]
pub struct Scheduler {
    /// Placeholder field
    _placeholder: (),
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Scheduler { _placeholder: () }
    }
    
    /// Schedule a task
    pub fn schedule(&self, _task: Box<dyn FnOnce()>) -> bool {
        true
    }
    
    /// Run the scheduler
    pub fn run(&self) -> bool {
        true
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}