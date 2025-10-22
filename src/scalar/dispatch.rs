use std::collections::VecDeque;
use tracing::debug;
use crate::scalar::instruction::{Instruction, InstructionType};
use crate::scalar::scoreboard::Scoreboard;
use crate::scalar::units::{AluUnit, BruUnit, LsuUnit};

/// Dispatch stage of the scalar pipeline
pub struct DispatchStage {
    pub queue: DispatchQueue,
    pub scoreboard: Scoreboard,
    pub alus: Vec<AluUnit>,
    pub brus: Vec<BruUnit>,
    pub lsu: LsuUnit,
    pub issue_width: u8,
}

impl DispatchStage {
    /// Create a new DispatchStage
    pub fn new() -> Self {
        Self {
            queue: DispatchQueue::new(8),
            scoreboard: Scoreboard::new(),
            alus: (0..4).map(|_| AluUnit::new()).collect(),
            brus: (0..4).map(|_| BruUnit::new()).collect(),
            lsu: LsuUnit::new(),
            issue_width: 4
        }
    }

    /// Tick the dispatch stage, dispatching up to 4 instructions
    pub fn tick(&mut self) {
        for unit in &mut self.alus {
            if let Some(rd) = unit.tick() {
                self.scoreboard.clear(rd);
                debug!("ALU writeback: clear rd={}", rd);
            }
        }

        for unit in &mut self.brus {
            if let Some(rd) = unit.tick() {
                self.scoreboard.clear(rd);
                debug!("BRU writeback: clear rd={}", rd);
            }
        }

        if let Some(rd) = self.lsu.tick() {
            self.scoreboard.clear(rd);
            debug!("LSU writeback: clear rd={}", rd);
        }

        if self.queue.inner.is_empty() {
            return;
        }

        let mut issued = 0usize;

        // Iterate through front instructions, but do not remove yet
        let mut to_remove = Vec::new();

        for (idx, instr) in self.queue.inner.iter().enumerate() {
            if issued as u8 >= self.issue_width {
                break;
            }

            if instr.typ == InstructionType::Unknown {
                to_remove.push(idx);
                continue;
            }

            let rs1_busy = self.scoreboard.is_busy(instr.rs1);
            let rs2_busy = self.scoreboard.is_busy(instr.rs2);
            let rd_busy = self.scoreboard.is_busy(instr.rd);

            if rs1_busy || rs2_busy || rd_busy {
                debug!(
                    "Stall instr {:?}: RAW/WAW hazard (rs1_busy={}, rs2_busy={}, rd_busy={})",
                    instr, rs1_busy, rs2_busy, rd_busy
                );
                continue;
            }

            let alu_len = self.alus.len();
            let bru_len = self.brus.len();

            match instr.typ {
                InstructionType::R | InstructionType::I => {
                    if let Some(unit) = self.alus.get_mut(issued % alu_len) {
                        self.scoreboard.set_busy(instr.rd);
                        unit.issue(instr);
                        issued += 1;
                        to_remove.push(idx);
                    }
                }
                InstructionType::B | InstructionType::J => {
                    if let Some(unit) = self.brus.get_mut(issued % bru_len) {
                        self.scoreboard.set_busy(instr.rd);
                        unit.issue(instr);
                        issued += 1;
                        to_remove.push(idx);
                    }
                }
                InstructionType::S => {
                    self.lsu.issue(instr);
                    issued += 1;
                    to_remove.push(idx);
                }
                _ => {}
            }
        }
        for &idx in to_remove.iter().rev() {
            self.queue.inner.remove(idx);
        }

        debug!("Issued {} instructions this cycle", issued);
    }
}

/// Dispatch queue holding instructions to be dispatched
pub struct DispatchQueue {
    pub inner: VecDeque<Instruction>,
    pub capacity: usize,
}

impl DispatchQueue {
    /// Create a new DispatchQueue with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Push an instruction into the dispatch queue
    /// Returns true if the instruction was successfully pushed, false if the queue is full
    pub fn push(&mut self, instr: Instruction) -> bool {
        if self.inner.len() < self.capacity {
            self.inner.push_back(instr);
            true
        } else {
            false
        }
    }
}
