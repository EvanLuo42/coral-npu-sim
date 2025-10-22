#[derive(Copy, Clone)]
pub struct Scoreboard {
    busy: [bool; 32]
}

impl Scoreboard {
    pub fn new() -> Self {
        Self { busy: [false; 32] }
    }

    /// Check if a register is busy
    pub fn is_busy(&self, reg: u8) -> bool {
        reg != 0 && self.busy[reg as usize]
    }

    /// Mark a register as busy (destination register)
    pub fn set_busy(&mut self, rd: u8) {
        if rd != 0 {
            self.busy[rd as usize] = true;
        }
    }

    /// Clear register when writeback completes
    pub fn clear(&mut self, rd: u8) {
        if rd != 0 {
            self.busy[rd as usize] = false;
        }
    }

    /// Clone for speculative check (multi-issue scan)
    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}
