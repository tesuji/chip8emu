#[cfg(test)]
mod tests;

const MAX_STACK: usize = 16;

pub struct Stack {
    storage: [u16; MAX_STACK],
    len: usize,
}

impl Stack {
    pub const fn new() -> Self {
        Self { len: 0, storage: [0; MAX_STACK] }
    }

    pub fn reset(&mut self) {
        self.storage = [0; MAX_STACK];
        self.len = 0;
    }

    #[must_use]
    pub fn push(&mut self, item: u16) -> Option<()> {
        if self.len >= MAX_STACK {
            None
        } else {
            self.storage[self.len] = item;
            self.len += 1;
            Some(())
        }
    }

    #[must_use]
    pub fn pop(&mut self) -> Option<u16> {
        if self.len > 0 && self.len < MAX_STACK {
            self.len -= 1;
            Some(self.storage[self.len])
        } else {
            None
        }
    }
}
