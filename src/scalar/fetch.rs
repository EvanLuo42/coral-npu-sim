use crate::common::io::{Future, Poll};
use crate::scalar::instruction::InstructionBuffer;
use crate::scalar::memory::{Itcm, ItcmRead};

pub struct FetchStage {
    pub pcs: [u32; 4],
    pub pending_reads: [Option<ItcmRead>; 4],
}

impl FetchStage {
    pub fn new() -> Self {
        Self {
            pcs: [0, 4, 8, 12],
            pending_reads: [None; 4],
        }
    }

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
