//!
//! Generate names for temporary variables.
//!

use era_yul::util::counter::Counter;

///
/// Generate names for temporary variables.
///
#[derive(Debug, Default)]
pub struct NameGenerator {
    counter: Counter,
}

impl NameGenerator {
    ///
    /// Create a new instance of [`NameGenerator`] with a starting state.
    ///
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }

    ///
    /// Generate a temporary variable name.
    ///
    pub fn new_variable(&mut self) -> String {
        let name = format!("tmp{}", self.counter.get_value());
        self.counter.increment();
        name
    }
}
