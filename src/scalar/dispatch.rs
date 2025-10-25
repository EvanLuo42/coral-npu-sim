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
            scoreboard: Scoreboard::new(4, 4),
            alus: (0..4).map(|_| AluUnit::new()).collect(),
            brus: (0..4).map(|_| BruUnit::new()).collect(),
            lsu: LsuUnit::new(),
            issue_width: 4
        }
    }

    /// Tick the dispatch stage, dispatching up to 4 instructions
    pub fn tick(&mut self) {
        let mut issued = 0;
        let mut remaining = VecDeque::new();

        debug!(
            "Queue size: {}, ALUs busy: {}/{}",
            self.queue.inner.len(),
            self.alus.iter().filter(|u| u.busy).count(),
            self.alus.len()
        );

        while issued < self.issue_width && let Some(instr) = self.queue.inner.pop_front() {
            if !self.scoreboard.can_issue(&instr) {
                debug!("Stall: data hazard detected for {}", instr);
                self.scoreboard.predict_issue(&instr);
                remaining.push_back(instr);
                continue;
            }

            if !self.scoreboard.allocate_unit(&instr) {
                debug!("Stall: no free execution unit for {}", instr);
                remaining.push_back(instr);
                continue;
            }

            self.scoreboard.mark_issue(&instr);
            debug!("Issued: {}", instr);

            match instr.opcode {
                0b0110011 | 0b0010011 => { // ALU
                    if let Some(unit) = self.alus.iter_mut().find(|u| !u.busy) {
                        unit.issue(instr);
                    }
                }
                0b1100011 => { // BRANCH
                    if let Some(unit) = self.brus.iter_mut().find(|u| !u.busy) {
                        unit.issue(instr);
                    }
                }
                0b0000011 | 0b0100011 => { // LOAD/STORE
                    if !self.lsu.busy {
                        self.lsu.issue(instr);
                    }
                }
                _ => {}
            }

            issued += 1;
        }

        if !remaining.is_empty() {
            debug!("Re-queue {} stalled instructions", remaining.len());
        }
        self.queue.inner = remaining;

        for alu in &mut self.alus {
            if let Some(done) = alu.tick() {
                debug!("ALU complete: {}", done);
                self.scoreboard.mark_complete(&done);
                self.scoreboard.release_unit(&done);
            }
        }
        for bru in &mut self.brus {
            if let Some(done) = bru.tick() {
                debug!("BRU complete: {}", done);
                self.scoreboard.mark_complete(&done);
                self.scoreboard.release_unit(&done);
            }
        }
        if let Some(done) = self.lsu.tick() {
            debug!("LSU complete: {}", done);
            self.scoreboard.mark_complete(&done);
            self.scoreboard.release_unit(&done);
        }
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
