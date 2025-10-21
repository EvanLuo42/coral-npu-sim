use std::collections::VecDeque;

#[derive(Copy, Clone, Default)]
pub struct RawInstruction {
    pub data: u32
}

#[derive(Copy, Clone, Debug)]
pub enum InstructionType {
    R, I, S, B, U, J, Unknown
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub imm: i32,
    pub typ: InstructionType,
}

impl From<RawInstruction> for Instruction {
    fn from(raw: RawInstruction) -> Self {
        let data = raw.data;
        let opcode = (data & 0x7F) as u8;
        let rd = ((data >> 7) & 0x1F) as u8;
        let funct3 = ((data >> 12) & 0x07) as u8;
        let rs1 = ((data >> 15) & 0x1F) as u8;
        let rs2 = ((data >> 20) & 0x1F) as u8;
        let funct7 = ((data >> 25) & 0x7F) as u8;

        let (typ, imm) = match opcode {
            0b0110011 => (InstructionType::R, 0), // add, sub, and, or, etc
            0b0010011 => (InstructionType::I, (data as i32) >> 20),
            0b0000011 => (InstructionType::I, (data as i32) >> 20), // load
            0b0100011 => {
                // store: imm[11:5 | 4:0]
                let imm = (((data >> 25) << 5) | ((data >> 7) & 0x1F)) as i32;
                (InstructionType::S, sign_extend(imm, 12))
            }
            0b1100011 => {
                // branch
                let imm = (((((data >> 31) & 0x1) << 12)
                    | (((data >> 7) & 0x1) << 11)
                    | (((data >> 25) & 0x3F) << 5)
                    | (((data >> 8) & 0xF) << 1)) as i32);
                (InstructionType::B, sign_extend(imm, 13))
            }
            0b0110111 | 0b0010111 => {
                // lui / auipc
                let imm = (data & 0xFFFFF000) as i32;
                (InstructionType::U, imm)
            }
            0b1101111 => {
                // jal
                let imm = ((((data >> 31) & 0x1) << 20)
                    | (((data >> 12) & 0xFF) << 12)
                    | (((data >> 20) & 0x1) << 11)
                    | (((data >> 21) & 0x3FF) << 1)) as i32;
                (InstructionType::J, sign_extend(imm, 21))
            }
            _ => (InstructionType::Unknown, 0),
        };

        Instruction {
            opcode,
            rd,
            rs1,
            rs2,
            funct3,
            funct7,
            imm,
            typ,
        }
    }
}

pub fn sign_extend(value: i32, bits: u8) -> i32 {
    let shift = 32 - bits;
    (value << shift) >> shift
}

pub struct InstructionBuffer {
    pub queue: VecDeque<RawInstruction>,
    pub capacity: usize,
}

impl InstructionBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, instr: RawInstruction) {
        if self.queue.len() < self.capacity {
            self.queue.push_back(instr);
        }
    }

    pub fn pop_batch(&mut self, n: usize) -> Vec<RawInstruction> {
        let mut out = Vec::new();
        for _ in 0..n {
            if let Some(instr) = self.queue.pop_front() {
                out.push(instr);
            } else {
                break;
            }
        }
        out
    }
}
