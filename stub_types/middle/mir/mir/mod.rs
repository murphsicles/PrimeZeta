/// Stub Mir type for v0.5.0 compilation
#[derive(Debug, Clone)]
pub struct Mir {
    /// Placeholder field
    _placeholder: (),
}

impl Mir {
    /// Create a new Mir
    pub fn new() -> Self {
        Mir { _placeholder: () }
    }
}

impl Default for Mir {
    fn default() -> Self {
        Self::new()
    }
}