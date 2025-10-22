use crate::common::io::{Future, Poll};
use crate::scalar::instruction::InstructionBuffer;
use crate::scalar::memory::{Itcm, ItcmRead};

/// The FetchStage struct represents the fetch stage of the scalar pipeline
pub struct FetchStage {
    pub pcs: [u32; 4],
    pub pending_reads: [Option<ItcmRead>; 4],
}

impl FetchStage {
    /// Creates a new FetchStage instance with initial program counters and empty pending reads
    pub fn new() -> Self {
        Self {
            pcs: [0, 4, 8, 12],
            pending_reads: [None; 4],
        }
    }

    /// Advances the fetch stage by one tick, fetching instructions from ITCM and pushing them to the instruction buffer
    pub fn tick(&mut self, instr_buffer: &mut InstructionBuffer, itcm: &mut Itcm) {
        for lane in 0..4 {
            match &mut self.pending_reads[lane] {
                None => {
                    self.pending_reads[lane] = Some(itcm.read(self.pcs[lane]));
                }
                Some(pending) => match pending.poll(itcm) {
                    Poll::Ready(instr) => {
                        self.pending_reads[lane] = None;
                        instr_buffer.push(instr);
                        self.pcs[lane] += 4 * 4;
                    }
                    Poll::Pending => {}
                },
            }
        }
    }
}
