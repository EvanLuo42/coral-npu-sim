use tracing::debug;
use crate::scalar::instruction::Instruction;

pub struct AluUnit {
    pub busy: bool,
    pub rd: Option<u8>,
    pub remaining: u8,
}

pub struct BruUnit {
    pub busy: bool,
    pub rd: Option<u8>,
    pub remaining: u8,
}

pub struct LsuUnit {
    pub busy: bool,
    pub rd: Option<u8>,
    pub remaining: u8,
}

impl AluUnit {
    pub fn new() -> Self {
        Self { busy: false, rd: None, remaining: 0 }
    }

    pub fn issue(&mut self, instr: &Instruction) {
        self.busy = true;
        self.rd = Some(instr.rd);
        self.remaining = 2;
        debug!("ALU executes {:?}", instr);
    }

    pub fn tick(&mut self) -> Option<u8> {
        if self.busy && self.remaining > 0 {
            self.remaining -= 1;
            if self.remaining == 0 {
                self.busy = false;
                return self.rd.take();
            }
        }
        None
    }
}

impl BruUnit {
    pub fn new() -> Self {
        Self { busy: false, rd: None, remaining: 0 }
    }

    pub fn issue(&mut self, instr: &Instruction) {
        self.busy = true;
        self.rd = Some(instr.rd);
        self.remaining = 2;
        debug!("BRU executes {:?}", instr);
    }

    pub fn tick(&mut self) -> Option<u8> {
        if self.busy && self.remaining > 0 {
            self.remaining -= 1;
            if self.remaining == 0 {
                self.busy = false;
                return self.rd.take();
            }
        }
        None
    }
}

impl LsuUnit {
    pub fn new() -> Self {
        Self { busy: false, rd: None, remaining: 0 }
    }

    pub fn issue(&mut self, instr: &Instruction) {
        self.busy = true;
        self.rd = Some(instr.rd);
        self.remaining = 2;
        debug!("ALU executes {:?}", instr);
    }

    pub fn tick(&mut self) -> Option<u8> {
        if self.busy && self.remaining > 0 {
            self.remaining -= 1;
            if self.remaining == 0 {
                self.busy = false;
                return self.rd.take();
            }
        }
        None
    }
}