#[cfg(test)]
mod tests;

use crate::alloc::boxed_zeroed_display;
use crate::num::BitIter;

pub const WIDTH: u16 = if cfg!(feature = "chip48") { 128 } else { 64 };
pub const HEIGHT: u16 = if cfg!(feature = "chip48") { 64 } else { 32 };

pub(crate) const DISPLAY_SIZE: usize = (WIDTH * HEIGHT) as usize;
const PIXELS_WIDE: u16 = 8;

pub struct Display {
    vram: Box<[bool; DISPLAY_SIZE]>,
}

impl Display {
    pub fn new() -> Self {
        Self { vram: boxed_zeroed_display() }
    }

    pub fn reset(&mut self) {
        self.vram.fill(false);
    }

    pub(crate) fn get_buf(&self) -> &[bool; DISPLAY_SIZE] {
        &self.vram
    }

    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels
    // and a height of N+1 pixels. Each row of 8 pixels is read as bit-coded
    // starting from memory location I; I value doesn't change after
    // the execution of this instruction. As described above, VF is set to 1
    // if any screen pixels are flipped from set to unset when
    // the sprite is drawn, and to 0 if that doesn't happen.
    #[must_use]
    pub fn draw(&mut self, (x, y): (u8, u8), sprites: &[u8]) -> bool {
        let mut collision = false;
        let (x, y) = (u16::from(x) % WIDTH, u16::from(y) % HEIGHT);
        let x_end = (x + PIXELS_WIDE).min(WIDTH);
        for (y, &b) in (y..HEIGHT).zip(sprites.iter()) {
            for (x, b) in (x..x_end).zip(BitIter::new(b)) {
                let coord = usize::from(x + y * WIDTH);
                collision |= self.vram[coord] & b;
                self.vram[coord] ^= b;
            }
        }
        collision
    }

    pub fn clear_screen(&mut self) {
        self.vram.fill(false);
    }
}
