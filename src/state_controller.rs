//! Collatz Base 6 CA State controller.

use piston::input::GenericEvent;

use crate::CCAState;

/// Handles events for the Collatz base 6 CA.
pub struct CCAStateController {
    /// Stores the CA state.
    pub state: CCAState,
}

impl CCAStateController {
    /// Creates a new state controller.
    pub fn new(state: CCAState) -> CCAStateController {
        CCAStateController {
            state: state,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}