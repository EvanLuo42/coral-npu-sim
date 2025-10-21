use crate::scalar::dispatch::DispatchQueue;
use crate::scalar::instruction::{Instruction, InstructionBuffer, RawInstruction};

pub struct DecodeStage {
    pub lanes: [Option<RawInstruction>; 4]
}

impl DecodeStage {
    pub fn new() -> Self {
        Self {
            lanes: [None; 4]
        }
    }

    pub fn accept_batch(&mut self, instrs: Vec<RawInstruction>) {
        for (i, instr) in instrs.into_iter().enumerate() {
            if i < 4 {
                self.lanes[i] = Some(instr);
            }
        }
    }

    pub fn tick(&mut self, instr_buffer: &mut InstructionBuffer, dispatch_q: &mut DispatchQueue) {
        let batch = instr_buffer.pop_batch(4);
        self.accept_batch(batch);
        
        for lane in 0..4 {
            if let Some(raw) = self.lanes[lane].take() {
                let decoded = Instruction::from(raw);
                if !dispatch_q.push(decoded) {
                    break;
                }
            }
        }
    }
}
