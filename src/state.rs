//! Collatz Base 6 CA State
use std::collections::VecDeque;

/// Stores the data of one cell
#[derive(Debug, Clone, Copy)]
pub struct TritBitDomino {
    /// trit
    pub trit_bit: [bool; 3],
    /// is part of configuration's tail?
    pub is_tail: bool,
}

impl TritBitDomino {
    /// Converts a trit bit domino to a base 6 digit
    pub fn to_base_6_digit(&self) -> char {
        return std::char::from_digit(
            2 * (self.trit_bit[0] as u32) + 2 * (self.trit_bit[1] as u32) + self.trit_bit[2] as u32,
            10,
        )
        .unwrap();
    }

    /// Converts a base 6 digit to a trit bit domino
    pub fn from_base_6_digit(digit: u8, is_tail: bool) -> Result<TritBitDomino, String> {
        if !(digit >= 0 && digit < 6) {
            return Err(format!("Digit `{}` is not a valid base 6 digit.", digit));
        }
        let trit_carry: bool = digit >= 2 && digit != 3;
        let trit_sum: bool = digit >= 3;
        return Ok(TritBitDomino {
            trit_bit: [trit_carry, trit_sum, digit % 2 != 0],
            is_tail,
        });
    }
}

/// Stores Collatz CA state.
#[derive(Debug, Clone)]
pub struct CCAState {
    /// State representation
    pub cells: VecDeque<TritBitDomino>,

    /// Initial configuration
    pub init_str: String,
}

impl CCAState {
    /// Create a new CCAState from base 6 string
    pub fn from_str(base_6_str: &str) -> Result<CCAState, String> {
        if !CCAState::check_base_6(base_6_str) {
            return Err(format!(
                "Input `{}` is not a valid base 6 string.",
                base_6_str
            ));
        }

        let mut cells: VecDeque<TritBitDomino> = VecDeque::new();
        for c in base_6_str.chars() {
            let tbd = CCAState::b6_to_tbd(c)?;
            cells.push_back(tbd);
        }
        cells.back_mut().unwrap().is_tail = true;
        Ok(CCAState {
            cells,
            init_str: base_6_str.to_string(),
        })
    }

    /// Reset state to initial state
    pub fn reset(&mut self) -> () {
        self.cells = CCAState::from_str(&self.init_str[..])
            .unwrap()
            .cells
            .clone();
    }

    /// Flushes cells on the tail
    pub fn flush_tail(&mut self) -> () {
        let mut can_flush = false;
        for tbd in self.cells.iter().rev() {
            if !tbd.is_tail {
                break;
            }
            if tbd.trit_bit[2] {
                can_flush = true;
                break;
            }
        }

        if can_flush {
            while !self.cells.back().unwrap().trit_bit[2] {
                self.cells.pop_back();
            }
        }
    }

    /// Translate current state to base 6 string
    pub fn to_str(&self) -> String {
        let mut to_return: String = String::new();
        for tbd in self.cells.iter() {
            to_return.push(tbd.to_base_6_digit());
        }
        to_return
    }

    /// Check if a given string is valid base 6
    fn check_base_6(base_6_str: &str) -> bool {
        println!("The string: {}", base_6_str);
        for c in base_6_str.chars() {
            if !((c >= '0') && (c <= '5')) {
                return false;
            }
        }
        true
    }

    /// Translate base 6 digit to trit-bit domino
    fn b6_to_tbd(char_base_6_digit: char) -> Result<TritBitDomino, String> {
        if !(char_base_6_digit >= '0' && char_base_6_digit <= '5') {
            return Err(format!(
                "`{}` is not a valid base 6 digit.",
                char_base_6_digit
            ));
        }

        let digit = (char_base_6_digit as u8) - ('0' as u8);
        let trit_bit_domino = TritBitDomino::from_base_6_digit(digit, false)?;
        return Ok(trit_bit_domino);
    }
}
