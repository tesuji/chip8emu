use std::convert::TryFrom;
use std::time::Instant;

// In COSMAC VIP manual, this is the minimum value that the timer responds
const MIN_SOUND_DURATION: u8 = 2;
const FPS: u32 = 60;
const NANOS_PER_SECOND: u32 = 10_u32.pow(9);
const WAIT_TIME_NS: u32 = NANOS_PER_SECOND / FPS;

/// Intended to be used for timing the events of games. Its value can be set and read.
pub struct DelayTimer {
    value: u8,
    timer: Instant,
}

/// Used for sound effects. When its value is nonzero, a beeping sound is made.
pub struct SoundTimer(u8);

impl DelayTimer {
    pub fn new() -> Self {
        Self { value: 0, timer: Instant::now() }
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn load(&mut self) -> u8 {
        if self.value > 0 {
            let nanos = self.timer.elapsed().as_nanos();
            let div = nanos / (WAIT_TIME_NS as u128);
            if div > 0 {
                self.value = match u8::try_from(div) {
                    Ok(c) => self.value.saturating_sub(c),
                    Err(_) => 0,
                };
                self.timer = Instant::now();
            }
            self.value
        } else {
            0
        }
    }

    pub fn store(&mut self, time: u8) {
        self.value = time;
        self.timer = Instant::now();
    }

    #[cfg(FALSE)]
    pub fn decrease(&mut self) {
        self.value = self.value.saturating_sub(1);
    }
}

impl SoundTimer {
    pub const fn new() -> Self {
        Self(0)
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

    pub fn store(&mut self, time: u8) {
        assert!(time >= MIN_SOUND_DURATION);
        self.0 = time;
    }

    pub fn decrease(&mut self) {
        // unimplemented!("Make a beep");
        // eprintln!("BEEP");
        self.0 = self.0.saturating_sub(1);
    }
}
