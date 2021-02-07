use std::fmt;

pub struct Opcode(u16);

pub enum OpcodeKind {
    JpAddr {
        addr: u16,
    },
    JpVxAddr {
        #[cfg(not(feature = "original"))]
        x: u8,
        addr: u16,
    },
    Ret,
    Call {
        addr: u16,
    },
    SkipVxByte {
        eq: bool,
        x: u8,
        byte: u8,
    },
    SkipVxVy {
        eq: bool,
        x: u8,
        y: u8,
    },
    LoadVxByte {
        x: u8,
        byte: u8,
    },
    AddVxByte {
        x: u8,
        byte: u8,
    },
    LoadVxVy {
        x: u8,
        y: u8,
    },
    Or {
        x: u8,
        y: u8,
    },
    And {
        x: u8,
        y: u8,
    },
    Xor {
        x: u8,
        y: u8,
    },
    Add {
        x: u8,
        y: u8,
    },
    Subtract {
        x_y: bool,
        x: u8,
        y: u8,
    },
    ShiftRight {
        x: u8,
        y: u8,
    },
    ShiftLeft {
        x: u8,
        y: u8,
    },
    Random {
        x: u8,
        byte: u8,
    },
    LoadDT {
        x: u8,
    },
    StoreDT {
        x: u8,
    },
    StoreST {
        x: u8,
    },
    LoadK {
        x: u8,
    },
    SkipIfKey {
        eq: bool,
        x: u8,
    },
    LoadI {
        addr: u16,
    },
    AddIVx {
        x: u8,
    },
    LoadBcd {
        x: u8,
    },
    PushRegs {
        x: u8,
    },
    PopRegs {
        x: u8,
    },
    Cls,
    Draw {
        x: u8,
        y: u8,
        n: u8,
    },
    LoadFont {
        x: u8,
    },
}

impl Opcode {
    pub fn new(op: u16) -> Self {
        Self(op)
    }

    pub fn decode(&self) -> OpcodeKind {
        use OpcodeKind::*;
        let nibbles = crate::num::to_4_be_nibles(self.0);
        let addr = self.0 & 0x0FFF;
        let kk = (self.0 & 0x00FF) as u8;

        match nibbles {
            /* Jumps */
            // JP addr
            [1, ..] => JpAddr { addr },
            // JP V0, addr
            #[cfg(feature = "original")]
            [0xb, ..] => JpVxAddr { addr },
            #[cfg(not(feature = "original"))]
            [0xb, x, ..] => JpVxAddr { x, addr },

            /* Subroutines */
            // RET
            [0x0, 0x0, 0xE, 0xE] => Ret,
            // CALL addr
            [2, ..] => Call { addr },

            /* Conditional branching */
            // SE Vx, byte
            [3, x, ..] => SkipVxByte { eq: true, x, byte: kk },
            // SNE Vx, byte
            [4, x, ..] => SkipVxByte { eq: false, x, byte: kk },
            // SE Vx, Vy
            [5, x, y, 0] => SkipVxVy { eq: true, x, y },
            // SNE Vx, Vy
            [9, x, y, 0] => SkipVxVy { eq: false, x, y },

            /* Storage */
            // LD Vx, byte
            [6, x, ..] => LoadVxByte { x, byte: kk },
            // ADD Vx, byte
            [7, x, ..] => AddVxByte { x, byte: kk },
            // LD Vx, Vy
            [8, x, y, 0] => LoadVxVy { x, y },

            // BIT operations
            // OR Vx, Vy
            [8, x, y, 1] => Or { x, y },
            // AND Vx, Vy
            [8, x, y, 2] => And { x, y },
            // XOR Vx, Vy
            [8, x, y, 3] => Xor { x, y },

            // MATH
            // ADD Vx, Vy
            [8, x, y, 4] => Add { x, y },
            // SUB Vx, Vy
            [8, x, y, 5] => Subtract { x_y: true, x, y },
            // SUBN Vx, Vy
            [8, x, y, 7] => Subtract { x_y: false, x, y },

            // SHIFT ops
            // SHR Vx {, Vy}
            [8, x, y, 6] => ShiftRight { x, y },
            // SHL Vx {, Vy}
            [8, x, y, 0xE] => ShiftLeft { x, y },

            /* Random */
            // RND Vx, byte
            [0xc, x, ..] => Random { x, byte: kk },

            /* Timers */
            // LD Vx, DT
            [0xf, x, 0, 7] => LoadDT { x },
            // LD DT, Vx
            [0xf, x, 1, 5] => StoreDT { x },
            // LD ST, Vx
            [0xf, x, 1, 8] => StoreST { x },

            /* KeyState Input */
            // LD Vx, K
            [0xf, x, 0, 0xa] => LoadK { x },
            // SKP Vx
            [0xe, x, 9, 0xe] => SkipIfKey { eq: true, x },
            // SKNP Vx
            [0xe, x, 0xa, 1] => SkipIfKey { eq: false, x },

            /* The I Register for graphics */
            // LD I, addr
            [0xa, ..] => LoadI { addr },
            // ADD I, Vx
            [0xf, x, 1, 0xe] => AddIVx { x },
            // LD B, Vx
            [0xf, x, 3, 3] => LoadBcd { x },
            // LD [I], Vx
            [0xf, x, 5, 5] => PushRegs { x },
            // Fx65 - LD Vx, [I]
            [0xf, x, 6, 5] => PopRegs { x },

            /* Graphics */
            // CLS
            [0x0, 0x0, 0xE, 0x0] => Cls,
            // DRW Vx, Vy, nibble
            [0xd, x, y, n] => Draw { x, y, n },
            // LD F, Vx
            [0xf, x, 2, 9] => LoadFont { x },

            // SYS addr
            [0, ..] => panic!("0NNN - `SYS addr` deprecated"),
            _ => unreachable!("no such instruction: {:X}", self),
        }
    }
}

impl fmt::UpperHex for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl fmt::Debug for OpcodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use OpcodeKind::*;
        match self {
            JpAddr { addr } => write!(f, "JMP {:#X}", addr),
            #[cfg(feature = "original")]
            JpVxAddr { addr } => write!(f, "JMP V0, {:#X}", addr),
            #[cfg(not(feature = "original"))]
            JpVxAddr { x, addr } => write!(f, "JMP V{:X}, {:#X}", x, addr),
            Ret => f.write_str("ret"),
            Call { addr } => write!(f, "CALL {:#X}", addr),
            SkipVxByte { eq, x, byte } => match eq {
                true => write!(f, "SE V{:X}, {:#X}", x, byte),
                false => write!(f, "SNE V{:X}, {:#X}", x, byte),
            },
            SkipVxVy { eq, x, y } => match eq {
                true => write!(f, "SE V{:X}, V{:X}", x, y),
                false => write!(f, "SNE V{:X}, V{:X}", x, y),
            },
            LoadVxByte { x, byte } => write!(f, "LD V{:X}, {:#X}", x, byte),
            AddVxByte { x, byte } => write!(f, "ADD V{:X}, {:#X}", x, byte),
            LoadVxVy { x, y } => write!(f, "LD V{:X}, V{:X}", x, y),
            Or { x, y } => write!(f, "OR V{:X}, V{:X}", x, y),
            And { x, y } => write!(f, "AND V{:X}, V{:X}", x, y),
            Xor { x, y } => write!(f, "XOR V{:X}, V{:X}", x, y),
            Add { x, y } => write!(f, "ADD V{:X}, V{:X}", x, y),
            Subtract { x_y, x, y } => match x_y {
                true => write!(f, "SUB V{:X}, V{:X}", x, y),
                false => write!(f, "SUBN V{:X}, V{:X}", x, y),
            },
            ShiftRight { x, y } => write!(f, "SHR V{:X} {{, V{:X}}}", x, y),
            ShiftLeft { x, y } => write!(f, "SHL V{:X} {{, V{:X}}}", x, y),
            Random { x, byte } => write!(f, "RND V{:X}, {:#X}", x, byte),
            LoadDT { x } => write!(f, "LD V{:X}, DT", x),
            StoreDT { x } => write!(f, "LD DT, V{:X}", x),
            StoreST { x } => write!(f, "LD ST, V{:X}", x),
            LoadK { x } => write!(f, "LD V{:X}, K", x),
            SkipIfKey { eq, x } => match eq {
                true => write!(f, "SKP V{:X}", x),
                false => write!(f, "SKNP V{:X}", x),
            },
            LoadI { addr } => write!(f, "LD I, {:#X}", addr),
            AddIVx { x } => write!(f, "ADD I, V{:X}", x),
            LoadBcd { x } => write!(f, "LD B, V{:X}", x),
            PushRegs { x } => write!(f, "LD [I], V{:X}", x),
            PopRegs { x } => write!(f, "LD V{:X}, [I]", x),
            Cls => f.write_str("CLS"),
            Draw { x, y, n } => write!(f, "DRW V{:X}, V{:X}, {:#X}", x, y, n),
            LoadFont { x } => write!(f, "LD F, V{:X}", x),
        }
    }
}
