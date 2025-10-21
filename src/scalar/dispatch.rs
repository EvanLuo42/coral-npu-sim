use std::collections::VecDeque;
use tracing::debug;
use crate::scalar::instruction::Instruction;

pub struct DispatchStage {
    pub queue: DispatchQueue,
}

impl DispatchStage {
    pub fn new() -> Self {
        Self {
            queue: DispatchQueue::new(8),
        }
    }

    pub fn tick(&mut self) {
        for _ in 0..4 {
            if let Some(instr) = self.queue.inner.pop_front() {
                debug!("Dispatching {:?}", instr);
                // todo!("Dispatch to ALU/BRU/LSU/Vector based on instr.typ")
            }
        }
    }
}

pub struct DispatchQueue {
    pub inner: VecDeque<Instruction>,
    pub capacity: usize,
}

impl DispatchQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, instr: Instruction) -> bool {
        if self.inner.len() < self.capacity {
            self.inner.push_back(instr);
            true
        } else {
            false
        }
    }
}
