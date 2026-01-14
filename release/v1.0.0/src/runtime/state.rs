use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeState {
    Stopped,
    Starting,
    Running,
    Paused,
    Error,
}

impl fmt::Display for RuntimeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl RuntimeState {
    /// Returns true if the transition from `self` to `target` is valid.
    pub fn can_transition_to(&self, target: &RuntimeState) -> bool {
        match (self, target) {
            // Initial startup
            (RuntimeState::Stopped, RuntimeState::Starting) => true,
            
            // Bootstrapping success
            (RuntimeState::Starting, RuntimeState::Running) => true,
            
            // Bootstrapping failure
            (RuntimeState::Starting, RuntimeState::Error) => true,
            
            // User pause
            (RuntimeState::Running, RuntimeState::Paused) => true,
            
            // System fault
            (RuntimeState::Running, RuntimeState::Error) => true,
            
            // User resume
            (RuntimeState::Paused, RuntimeState::Running) => true,
            
            // Reset after error
            (RuntimeState::Error, RuntimeState::Stopped) => true,
            
            // Shutdown from any state (except already stopped)
            (s, RuntimeState::Stopped) if *s != RuntimeState::Stopped => true,
            
            // No other transitions allowed
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(RuntimeState::Stopped.can_transition_to(&RuntimeState::Starting));
        assert!(RuntimeState::Starting.can_transition_to(&RuntimeState::Running));
        assert!(RuntimeState::Running.can_transition_to(&RuntimeState::Paused));
        assert!(RuntimeState::Paused.can_transition_to(&RuntimeState::Running));
        assert!(RuntimeState::Running.can_transition_to(&RuntimeState::Error));
        assert!(RuntimeState::Error.can_transition_to(&RuntimeState::Stopped));
        assert!(RuntimeState::Starting.can_transition_to(&RuntimeState::Stopped));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!RuntimeState::Stopped.can_transition_to(&RuntimeState::Running));
        assert!(!RuntimeState::Paused.can_transition_to(&RuntimeState::Starting));
        assert!(!RuntimeState::Error.can_transition_to(&RuntimeState::Running));
    }
}
