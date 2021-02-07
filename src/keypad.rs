use std::convert::TryFrom;
use std::mem;
use std::ops::{Index, IndexMut};

const KEYCODE_SIZE: usize = 16;

#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum KeyCode {
    K0 = 0, K1, K2, K3,
    K4, K5, K6, K7,
    K8, K9, KA, KB,
    KC, KD, KE, KF,
}

pub struct KeyState([bool; KEYCODE_SIZE]);

impl KeyState {
    pub fn new() -> Self {
        Self([false; KEYCODE_SIZE])
    }

    pub fn reset(&mut self) {
        self.0.fill(false);
    }

    #[must_use]
    pub fn any(&self) -> Option<u8> {
        self.0.iter().position(|&k| k != false).map(|x| x as u8)
    }

    #[must_use]
    pub fn key_down(&self, x: u8) -> bool {
        let kc = KeyCode::try_from(x).unwrap();
        self[kc]
    }
}

impl PartialEq<u8> for KeyCode {
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }
}

impl TryFrom<u8> for KeyCode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if usize::from(value) < KEYCODE_SIZE {
            Ok(unsafe { mem::transmute(value) })
        } else {
            Err(())
        }
    }
}

impl Index<KeyCode> for KeyState {
    type Output = bool;
    fn index(&self, kc: KeyCode) -> &Self::Output {
        &self.0[kc as usize]
    }
}

impl IndexMut<KeyCode> for KeyState {
    fn index_mut(&mut self, kc: KeyCode) -> &mut Self::Output {
        &mut self.0[kc as usize]
    }
}
