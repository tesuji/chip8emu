use std::mem::size_of;

use nanorand::{WyRand, RNG};

use super::display::{Display, DISPLAY_SIZE};
use super::keypad::{KeyCode, KeyState};
use super::memory::Memory;
use super::opcode::OpcodeKind;
use super::register::Registers;
use super::stack::Stack;
use super::timer::{DelayTimer, SoundTimer};

pub enum CpuState {
    Running,
    Step,
    Paused,
}

pub struct Cpu {
    memory: Memory,
    v: Registers,
    stack: Stack,
    delay_timer: DelayTimer,
    sound_timer: SoundTimer,
    // Peripherals
    keypad: KeyState,
    display: Display,
    randgen: WyRand,
    pub state: CpuState,
    should_draw: bool,
}

const _: &str = match size_of::<Cpu>() {
    #[cfg(target_vendor = "apple")]
    128 => "",
    #[cfg(not(target_vendor = "apple"))]
    136 => "",
    x => ["size of Cpu != 136"][x],
};

impl Cpu {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            v: Registers::zero(),
            stack: Stack::new(),
            delay_timer: DelayTimer::new(),
            sound_timer: SoundTimer::new(),
            display: Display::new(),
            keypad: KeyState::new(),
            state: CpuState::Running,
            randgen: WyRand::new(),
            should_draw: true,
        }
    }

    // #[cfg(FALSE)]
    pub fn reset(&mut self) {
        self.memory.reset();
        self.v.reset();
        self.stack.reset();
        self.delay_timer.reset();
        self.sound_timer.reset();
        self.display.reset();
        self.keypad.reset();
        self.state = CpuState::Running;
        self.should_draw = true;
    }

    pub fn load_game(&mut self, text: &[u8]) -> Result<(), crate::LoadError> {
        self.memory.load_program(text)
    }

    pub fn execute_cycle(&mut self) -> bool {
        self.should_draw = false;
        let opcode = self.memory.fetch();
        let kind = opcode.decode();
        #[cfg(FALSE)]
        eprintln!(
            "0x{:04X} - {:?}\n\
            {:?}",
            self.memory.pc.as_u16(),
            &kind,
            &self.v,
        );
        self.execute(kind);
        // Update timers
        // self.delay_timer.decrease();
        self.sound_timer.decrease();
        self.should_draw
    }

    pub fn set_key_state(&mut self, kc: KeyCode, pressed: bool) {
        self.keypad[kc] = pressed;
    }

    pub fn get_vram(&self) -> &[bool; DISPLAY_SIZE] {
        self.display.get_buf()
    }

    fn execute(&mut self, kind: OpcodeKind) {
        use OpcodeKind::*;
        match kind {
            /* Jump */
            JpAddr { addr } => self.memory.pc.set(addr),
            #[cfg(feature = "original")]
            JpVxAddr { addr } => self.memory.pc.set(addr.wrapping_add(self.v[0].into())),
            #[cfg(not(feature = "original"))]
            JpVxAddr { x, addr } => self.memory.pc.set(addr.wrapping_add(self.v[x].into())),

            /* Subroutines */
            Ret => self.call_back(),
            Call { addr } => self.call_to(addr),

            /* Conditional branching */
            SkipVxByte { eq, x, byte } => match (eq, self.v[x] == byte) {
                (true, true) => self.memory.pc.skip_next(),
                (false, false) => self.memory.pc.skip_next(),
                _ => {}
            },
            SkipVxVy { eq, x, y } => match (eq, self.v[x] == self.v[y]) {
                (true, true) => self.memory.pc.skip_next(),
                (false, false) => self.memory.pc.skip_next(),
                _ => {}
            },

            /* Storage */
            LoadVxByte { x, byte } => self.v[x] = byte,
            AddVxByte { x, byte } => {
                self.v[x] = self.v[x].wrapping_add(byte);
            }
            LoadVxVy { x, y } => self.v[x] = self.v[y],

            /* BIT operations */
            Or { x, y } => self.v[x] |= self.v[y],
            And { x, y } => self.v[x] &= self.v[y],
            Xor { x, y } => self.v[x] ^= self.v[y],

            /* MATH */
            Add { x, y } => {
                let (n, flag) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = n;
                self.v.set_vf(match flag {
                    true => 1,
                    false => 0,
                });
            }
            Subtract { x_y, x, y } => {
                let (n, flag) = match x_y {
                    true => self.v[x].overflowing_sub(self.v[y]),
                    false => self.v[y].overflowing_sub(self.v[x]),
                };
                self.v[x] = n;
                self.v.set_vf(match flag {
                    true => 0,
                    false => 1,
                });
            }

            /* SHIFT operations */
            ShiftRight { x, y } => {
                let (shifted, bit) = {
                    let y = if cfg!(feature = "original") {
                        self.v[y]
                    } else {
                        let _ = y;
                        self.v[x]
                    };
                    (y >> 1, y & 0x1)
                };
                self.v.set_vf(bit);
                self.v[x] = shifted;
            }
            ShiftLeft { x, y } => {
                let (shifted, bit) = {
                    let y = if cfg!(feature = "original") {
                        self.v[y]
                    } else {
                        let _ = y;
                        self.v[x]
                    };
                    (y << 1, (y & 0b1000_0000) >> 7)
                };
                self.v.set_vf(bit);
                self.v[x] = shifted;
            }

            /* Random */
            Random { x, byte } => self.v[x] = self.randgen.generate::<u8>() & byte,

            /* Timers */
            LoadDT { x } => self.v[x] = self.delay_timer.load(),
            StoreDT { x } => self.delay_timer.store(self.v[x]),
            StoreST { x } => self.sound_timer.store(self.v[x]),

            /* KeyState Input */
            LoadK { x } => match self.keypad.any() {
                Some(k) => self.v[x] = k,
                None => {
                    self.state = CpuState::Paused;
                }
            },
            SkipIfKey { eq, x } => match (eq, self.keypad.key_down(self.v[x])) {
                (true, true) => self.memory.pc.skip_next(),
                (false, false) => self.memory.pc.skip_next(),
                _ => {}
            },

            /* The I Register for graphics */
            LoadI { addr } => self.memory.i.store(addr),
            AddIVx { x } => {
                let _x = x;
                if cfg!(feature = "amiga") {
                    let f = self.memory.i.add_assign(self.v[_x]);
                    self.v.set_vf(u8::from(f));
                }
            }
            LoadBcd { x } => self.memory.store_bcd(self.v[x]),
            PushRegs { x } => self.regs_dump(x),
            PopRegs { x } => self.regs_load(x),

            /* Graphics */
            Cls => self.cls(),
            Draw { x, y, n } => self.draw((self.v[x], self.v[y]), n),
            LoadFont { x } => self.memory.i.set_to_builtin_fonts_addr(self.v[x]),
        }
    }

    fn regs_dump(&mut self, n: u8) {
        let up_to_vx = &self.v[..=n];
        self.memory.save_bytes_to_i(up_to_vx);
        #[cfg(feature = "original")]
        let _ = self.memory.i.add_assign(n + 1);
    }

    fn regs_load(&mut self, n: u8) {
        let dump = self.memory.read_bytes_from_i(n + 1);
        self.v[..=n].copy_from_slice(dump);
        #[cfg(feature = "original")]
        let _ = self.memory.i.add_assign(n + 1);
    }

    fn call_to(&mut self, addr: u16) {
        if let None = self.stack.push(self.memory.pc.as_u16()) {
            panic!("stack overflow");
        }
        self.memory.pc.set(addr);
    }

    fn call_back(&mut self) {
        let addr = match self.stack.pop() {
            Some(n) => n,
            None => panic!("stack is empty: cannot pop from stack"),
        };
        self.memory.pc.set(addr);
    }

    fn cls(&mut self) {
        self.should_draw = true;
        self.display.clear_screen();
    }

    fn draw(&mut self, (x, y): (u8, u8), n: u8) {
        let sprites = self.memory.read_bytes_from_i(n);
        let overlapped = u8::from(self.display.draw((x, y), sprites));
        self.v.set_vf(overlapped);
        self.should_draw = true;
    }
}
