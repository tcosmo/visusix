//! Collatz Base 6 CA State controller.

use piston::input::GenericEvent;

use crate::{CCAState, TritBitDomino};

/// Handles events for the Collatz base 6 CA.
pub struct CCAStateController {
    /// Stores the CA state.
    pub state: CCAState,

    /// Memorize all states CA went through
    pub memory_states: Vec<CCAState>,

    /// Where we are in `memory_states`
    pub memory_index: usize,
}

impl CCAStateController {
    /// Creates a new state controller.
    pub fn new(state: CCAState) -> CCAStateController {
        let state_clone = state.clone();
        CCAStateController {
            state: state,
            memory_states: vec![state_clone],
            memory_index: 0
        }
    }

    /// Set state back to previous state
    pub fn last(&mut self) -> () {

        if self.memory_index == 0 {
            return;
        }

        self.memory_index -= 1;
        self.state = self.memory_states[self.memory_index].clone();
    }

    /// Runs one step of the Collatz base 6 automaton
    pub fn next(&mut self) -> () {

        if self.memory_index+1 < self.memory_states.len() {
            self.memory_index += 1;
            self.state = self.memory_states[self.memory_index].clone();
            return;
        }

        if self.state.cells.len() == 0 {
            return;
        }

        ///self.state.flush_tail();

        let mut new_tbd_front: Option<TritBitDomino> = None;
        let tbd_front = self.state.cells.front().unwrap();

        if tbd_front.trit_bit[0] || tbd_front.trit_bit[1] {
            let carry =
                ((0 as u8) + (tbd_front.trit_bit[0] as u8) + (tbd_front.trit_bit[1] as u8)) >= 2;
            let sum = (((0 as u8) + (tbd_front.trit_bit[0] as u8) + (tbd_front.trit_bit[1] as u8))
                % 2)
                != 0;

            new_tbd_front = Some(TritBitDomino {
                trit_bit: [carry, false, sum],
                is_tail: (tbd_front.is_tail && !tbd_front.trit_bit[2]),
            })
        }

        let mut to_modify = Vec::new();
        let n = self.state.cells.len();
        for i in 0..n {
            let tbd: &TritBitDomino = self.state.cells.get(n - i - 1).unwrap();
            if tbd.is_tail {
                if tbd.trit_bit[2] {
                    let new_tbd = TritBitDomino {
                        trit_bit: [true, true, false],
                        is_tail: true,
                    };
                    to_modify.push((n - i - 1, new_tbd));
                }
            } else {
                let tbd_right: &TritBitDomino = self.state.cells.get(n - i).unwrap();
                let carry = ((tbd.trit_bit[2] as u8)
                    + (tbd_right.trit_bit[0] as u8)
                    + (tbd_right.trit_bit[1] as u8))
                    >= 2;
                let sum = (((tbd.trit_bit[2] as u8)
                    + (tbd_right.trit_bit[0] as u8)
                    + (tbd_right.trit_bit[1] as u8))
                    % 2)
                    != 0;

                let new_tbd = TritBitDomino {
                    trit_bit: [carry, tbd.trit_bit[2], sum],
                    is_tail: (tbd_right.is_tail && !tbd_right.trit_bit[2]),
                };

                to_modify.push((n - i - 1, new_tbd));
            }
        }

        for (i, tbd) in to_modify {
            *self.state.cells.get_mut(i).unwrap() = tbd;
        }

        if let Some(tbd) = new_tbd_front {
            self.state.cells.push_front(tbd);
        }

        self.memory_states.push(self.state.clone());
        self.memory_index += 1;
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key, MouseButton};
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::N => self.next(),
                Key::J => self.last(),
                Key::R => self.state.reset(),
                _ => {}
            }
        }
    }
}
