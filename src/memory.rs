#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use crate::alloc::boxed_zeroed_memory;
use crate::opcode::Opcode;

const ROM_START_ADDR: u16 = 0x200;
pub(crate) const RAM_SIZE: u16 = 1 << 12;
// const DISPLAY_REFRESH_START_ADDR: u16 = 0xF00;
const CALL_STACK_START_ADDR: u16 = 0xEA0;
const INSTRUCTION_SIZE: u16 = 2;
const FONTS_SET_ADDR: u16 = 0x0050;
const AVAILABLE_STORAGE: u16 = RAM_SIZE - ROM_START_ADDR;

/* Definitions */

pub struct Memory {
    pub pc: ProgramCounter,
    pub i: I,
    ram: Box<[u8; RAM_SIZE as usize]>,
}

/// Memory addresses containing the data for a given sprite (graphics).
pub struct I(u16);

#[derive(Copy, Clone)]
pub struct ProgramCounter(u16);

/* Implementations */

pub enum LoadError {
    RomTooBig(usize),
}

impl Memory {
    pub fn new() -> Self {
        let mut ram = boxed_zeroed_memory();
        const BEGIN: usize = FONTS_SET_ADDR as usize;
        const END: usize = BEGIN + FONTS_SET_LEN;
        ram[BEGIN..END].copy_from_slice(&FONTS_SET);
        Self { pc: ProgramCounter(ROM_START_ADDR), i: I(FONTS_SET_ADDR), ram }
    }

    pub fn reset(&mut self) {
        self.pc = ProgramCounter(ROM_START_ADDR);
        self.i = I(FONTS_SET_ADDR);
        self.ram.fill(0);
        const BEGIN: usize = FONTS_SET_ADDR as usize;
        const END: usize = BEGIN + FONTS_SET_LEN;
        self.ram[BEGIN..END].copy_from_slice(&FONTS_SET);
    }

    pub fn store_bcd(&mut self, x: u8) {
        let i = self.i.0 as usize;
        let abc = bcd(x);
        self.ram[i..(i + 3)].copy_from_slice(&abc[..]);
    }

    pub fn read_bytes_from_i(&self, n: u8) -> &[u8] {
        let i = self.i.0 as usize;
        let (begin, end) = (i, i + n as usize);
        &self.ram[begin..end]
    }

    pub fn save_bytes_to_i(&mut self, bytes: &[u8]) {
        let i = self.i.0 as usize;
        let (begin, end) = (i, i + bytes.len());
        self.ram[begin..end].copy_from_slice(bytes);
    }

    pub fn load_program(&mut self, text: &[u8]) -> Result<(), LoadError> {
        let len = text.len();
        if len >= AVAILABLE_STORAGE as usize {
            Err(LoadError::RomTooBig(len))
        } else {
            let (begin, end) = (ROM_START_ADDR as usize, ROM_START_ADDR as usize + len);
            self.ram[begin..end].copy_from_slice(text);
            Ok(())
        }
    }

    pub fn fetch(&mut self) -> Opcode {
        let pc = usize::from(self.pc.0);
        self.pc.0 += INSTRUCTION_SIZE;

        let mut op = [0; INSTRUCTION_SIZE as usize];
        op.copy_from_slice(&self.ram[pc..(pc + INSTRUCTION_SIZE as usize)]);
        Opcode::new(u16::from_be_bytes(op))
    }
}

impl ProgramCounter {
    pub fn as_u16(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, addr: u16) {
        assert!(addr >= ROM_START_ADDR);
        assert!(addr <= CALL_STACK_START_ADDR);
        self.0 = addr;
    }

    pub fn skip_next(&mut self) {
        self.0 += INSTRUCTION_SIZE;
    }

    #[cfg(FALSE)]
    pub fn decrease(&mut self) {
        self.0 -= INSTRUCTION_SIZE;
    }
}

impl I {
    #[must_use]
    #[cfg(feature = "amiga")]
    pub fn add_assign(&mut self, value: u8) -> bool {
        self.0 += u16::from(value);
        if cfg!(feature = "amiga") {
            self.0 < RAM_SIZE
        } else {
            assert!(self.0 < RAM_SIZE);
            false
        }
    }

    pub fn set_to_builtin_fonts_addr(&mut self, x: u8) {
        let x = if cfg!(feature = "original") {
            x & 0xF
        } else {
            assert!((x as u16) < TOTAL_HEXI);
            x
        };
        self.0 = FONTS_SET_ADDR + EACH_FONT_SIZE * (x as u16);
    }

    pub fn store(&mut self, addr: u16) {
        assert!(addr < RAM_SIZE);
        self.0 = addr;
    }
}

impl Error for LoadError {}
impl fmt::Debug for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::RomTooBig(n) => {
                write!(f, "loaded rom is too big: {} > {}", n, AVAILABLE_STORAGE)
            }
        }
    }
}
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn bcd(x: u8) -> [u8; 3] {
    [x / 100, x / 10 % 10, x % 10]
}

const EACH_FONT_SIZE: u16 = 5;
const TOTAL_HEXI: u16 = 16;
const FONTS_SET_LEN: usize = (EACH_FONT_SIZE * TOTAL_HEXI) as usize;
// Hexadecimal digits: 0-9A-F
pub(crate) static FONTS_SET: [u8; FONTS_SET_LEN] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
