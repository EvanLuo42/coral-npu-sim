use crate::common::io::{Future, Poll};
use crate::common::tick::Tickable;
use crate::scalar::instruction::RawInstruction;
use crate::scalar::memory::{Itcm, ItcmRead};

pub struct FetchStage {
    pub pc: u32,
    pub instr_buffer: Vec<RawInstruction>,
    pub pending_read: Option<ItcmRead>,
    pub itcm: Itcm
}

impl Tickable for FetchStage {
    fn tick(&mut self) {
        match self.pending_read {
            None => self.pending_read = Some(self.itcm.read(self.pc)),
            Some(mut pending) => {
                if let Poll::Ready(instr) = pending.poll(&mut self.itcm) {
                    self.pending_read = None;
                    self.instr_buffer.push(instr);
                    self.pc += 4;
                }
            }
        }
    }
}
