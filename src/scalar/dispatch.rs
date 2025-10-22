use std::collections::VecDeque;
use tracing::debug;
use crate::scalar::instruction::Instruction;

/// Dispatch stage of the scalar pipeline
pub struct DispatchStage {
    pub queue: DispatchQueue,
}

impl DispatchStage {
    /// Create a new DispatchStage
    pub fn new() -> Self {
        Self {
            queue: DispatchQueue::new(8),
        }
    }

    /// Tick the dispatch stage, dispatching up to 4 instructions
    pub fn tick(&mut self) {
        for _ in 0..4 {
            if let Some(instr) = self.queue.inner.pop_front() {
                debug!("Dispatching {:?}", instr);
                // todo!("Dispatch to ALU/BRU/LSU/Vector based on instr.typ")
            }
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
