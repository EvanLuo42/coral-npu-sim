use crate::scalar::instruction::Instruction;

pub struct AluUnit {
    pub busy: bool,
    pub remaining: u8,
    pub current: Option<Instruction>,
}

impl AluUnit {
    pub fn new() -> Self {
        Self { busy: false, remaining: 0, current: None }
    }

    pub fn issue(&mut self, instr: Instruction) {
        self.busy = true;
        self.remaining = 1;
        self.current = Some(instr);
    }

    pub fn tick(&mut self) -> Option<Instruction> {
        if self.busy {
            if self.remaining > 0 {
                self.remaining -= 1;
            } else {
                self.busy = false;
                return self.current.take();
            }
        }
        None
    }
}

pub struct BruUnit {
    pub busy: bool,
    pub remaining: u8,
    pub current: Option<Instruction>,
}

impl BruUnit {
    pub fn new() -> Self {
        Self { busy: false, remaining: 0, current: None }
    }

    pub fn issue(&mut self, instr: Instruction) {
        self.busy = true;
        self.remaining = 1;
        self.current = Some(instr);
    }

    pub fn tick(&mut self) -> Option<Instruction> {
        if self.busy {
            if self.remaining > 0 {
                self.remaining -= 1;
            } else {
                self.busy = false;
                return self.current.take();
            }
        }
        None
    }
}

pub struct LsuUnit {
    pub busy: bool,
    pub remaining: u8,
    pub current: Option<Instruction>,
}

impl LsuUnit {
    pub fn new() -> Self {
        Self { busy: false, remaining: 0, current: None }
    }

    pub fn issue(&mut self, instr: Instruction) {
        self.busy = true;
        self.remaining = 1;
        self.current = Some(instr);
    }

    pub fn tick(&mut self) -> Option<Instruction> {
        if self.busy {
            if self.remaining > 0 {
                self.remaining -= 1;
            } else {
                self.busy = false;
                return self.current.take();
            }
        }
        None
    }
}