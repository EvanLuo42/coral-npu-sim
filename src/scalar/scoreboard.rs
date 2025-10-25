use crate::scalar::instruction::Instruction;

/// Simple scoreboard for scalar pipeline.
/// Tracks register availability and functional unit busy states.
pub struct Scoreboard {
    pub reg_busy: [bool; 32], // x0..x31
    pub pending_busy: [bool; 32],
    pub alu_busy: Vec<bool>,
    pub bru_busy: Vec<bool>,
    pub lsu_busy: bool,
}

impl Scoreboard {
    /// Create a new empty scoreboard
    pub fn new(num_alus: usize, num_brus: usize) -> Self {
        Self {
            reg_busy: [false; 32],
            pending_busy: [false; 32],
            alu_busy: vec![false; num_alus],
            bru_busy: vec![false; num_brus],
            lsu_busy: false,
        }
    }

    /// Check if an instruction can be issued without hazard
    pub fn can_issue(&self, instr: &Instruction) -> bool {
        let rs1_busy = instr.rs1 != 0 && (self.reg_busy[instr.rs1 as usize] || self.pending_busy[instr.rs1 as usize]);
        let rs2_busy = instr.rs2 != 0 && (self.reg_busy[instr.rs2 as usize] || self.pending_busy[instr.rs2 as usize]);
        let rd_waw = instr.rd != 0 && self.reg_busy[instr.rd as usize];
        !(rs1_busy || rs2_busy || rd_waw)
    }

    /// Mark destination register as busy
    pub fn mark_issue(&mut self, instr: &Instruction) {
        if instr.rd != 0 {
            self.pending_busy[instr.rd as usize] = false;
            self.reg_busy[instr.rd as usize] = true;
        }
    }

    pub fn predict_issue(&mut self, instr: &Instruction) {
        if instr.rd != 0 {
            self.pending_busy[instr.rd as usize] = true;
        }
    }

    /// Mark destination register as ready after writeback
    pub fn mark_complete(&mut self, instr: &Instruction) {
        if instr.rd != 0 {
            self.reg_busy[instr.rd as usize] = false;

        }
    }

    /// Find a free ALU unit (returns index)
    pub fn find_free_alu(&self) -> Option<usize> {
        self.alu_busy.iter().position(|b| !*b)
    }

    /// Find a free BRU unit
    pub fn find_free_bru(&self) -> Option<usize> {
        self.bru_busy.iter().position(|b| !*b)
    }

    /// Allocate a functional unit
    pub fn allocate_unit(&mut self, instr: &Instruction) -> bool {
        match instr.opcode {
            0b0110011 | 0b0010011 => { // ALU
                if let Some(i) = self.find_free_alu() {
                    self.alu_busy[i] = true;
                    return true;
                }
            }
            0b1100011 => { // BRANCH
                if let Some(i) = self.find_free_bru() {
                    self.bru_busy[i] = true;
                    return true;
                }
            }
            0b0000011 | 0b0100011 => { // LOAD / STORE
                if !self.lsu_busy {
                    self.lsu_busy = true;
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    /// Free a functional unit (called after execution done)
    pub fn release_unit(&mut self, instr: &Instruction) {
        match instr.opcode {
            0b0110011 | 0b0010011 => {
                if let Some(i) = self.alu_busy.iter().position(|b| *b) {
                    self.alu_busy[i] = false;
                }
            }
            0b1100011 => {
                if let Some(i) = self.bru_busy.iter().position(|b| *b) {
                    self.bru_busy[i] = false;
                }
            }
            0b0000011 | 0b0100011 => {
                self.lsu_busy = false;
            }
            _ => {}
        }
    }
}