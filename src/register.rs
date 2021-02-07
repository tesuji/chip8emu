use std::fmt;
use std::ops::{Index, IndexMut, RangeToInclusive};

const REG_TOTAL: usize = 16;

pub struct Registers {
    v: [u8; REG_TOTAL],
}

impl Registers {
    pub const fn zero() -> Self {
        Self { v: [0; REG_TOTAL] }
    }

    pub fn set_vf(&mut self, value: u8) {
        self.v[0xF] = value;
    }

    pub fn reset(&mut self) {
        self.v.fill(0);
    }
}

impl Index<u8> for Registers {
    type Output = u8;
    fn index(&self, x: u8) -> &Self::Output {
        &self.v[usize::from(x)]
    }
}

impl IndexMut<u8> for Registers {
    fn index_mut(&mut self, x: u8) -> &mut Self::Output {
        &mut self.v[usize::from(x)]
    }
}

impl Index<RangeToInclusive<u8>> for Registers {
    type Output = [u8];
    fn index(&self, r: RangeToInclusive<u8>) -> &Self::Output {
        &self.v[..=usize::from(r.end)]
    }
}

impl IndexMut<RangeToInclusive<u8>> for Registers {
    fn index_mut(&mut self, r: RangeToInclusive<u8>) -> &mut Self::Output {
        &mut self.v[..=usize::from(r.end)]
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[V0  V1  V2  V3  V4  V5  V6  V7  V8  V9  VA  VB  VC  VD  VE  VF]\n\
             [\
             {:02X}  {:02X}  {:02X}  {:02X}  \
             {:02X}  {:02X}  {:02X}  {:02X}  \
             {:02X}  {:02X}  {:02X}  {:02X}  \
             {:02X}  {:02X}  {:02X}  {:02X}\
             ]",
            self.v[0],
            self.v[1],
            self.v[2],
            self.v[3],
            self.v[4],
            self.v[5],
            self.v[6],
            self.v[7],
            self.v[8],
            self.v[9],
            self.v[10],
            self.v[11],
            self.v[12],
            self.v[13],
            self.v[14],
            self.v[15],
        )
    }
}
