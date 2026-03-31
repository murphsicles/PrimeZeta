/// Stub channel type for v0.5.0 compilation
#[derive(Debug)]
pub struct Channel<T> {
    /// Placeholder field
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Channel<T> {
    /// Create a new channel
    pub fn new() -> Self {
        Channel {
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Send a value through the channel
    pub fn send(&self, _value: T) -> bool {
        true
    }
    
    /// Receive a value from the channel
    pub fn recv(&self) -> Option<T> {
        None
    }
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}