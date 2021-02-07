#[cfg(test)]
mod tests;

use std::iter::ExactSizeIterator;

pub struct BitIter {
    value: u8,
    nth: u8,
}

/* Implementations */

pub const fn to_4_be_nibles(op: u16) -> [u8; 4] {
    // Q: why not use `u16::to_be_bytes` and continue from there?
    // A: Worse codegen: <https://rust.godbolt.org/z/Wz3GWf>.
    [
        ((op & 0xF000) >> 12) as u8,
        ((op & 0x0F00) >> 8) as u8,
        ((op & 0x00F0) >> 4) as u8,
        (op & 0x000F) as u8,
    ]
}

impl BitIter {
    const BITS: usize = 8;
    pub fn new(value: u8) -> Self {
        Self { value, nth: 0 }
    }
}

impl Iterator for BitIter {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.nth >= 8 {
            None
        } else {
            let mask = 0b1000_0000 >> self.nth;
            self.nth += 1;
            Some((mask & self.value) != 0)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (Self::BITS, Some(Self::BITS))
    }
}

impl ExactSizeIterator for BitIter {
    fn len(&self) -> usize {
        Self::BITS
    }
}
