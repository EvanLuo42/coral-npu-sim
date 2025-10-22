use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};

/// A raw RISC-V instruction.
#[derive(Copy, Clone, Default)]
pub struct RawInstruction {
    pub data: u32
}

/// The type of RISC-V instruction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InstructionType {
    R, I, S, B, U, J, Unknown
}

/// A decoded RISC-V instruction.
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

impl Instruction {
    /// Get the mnemonic of the instruction.
    fn mnemonic(&self) -> &'static str {
        match (self.opcode, self.funct3, self.funct7) {
            (0b0110011, 0b000, 0b0000000) => "add",
            (0b0110011, 0b000, 0b0100000) => "sub",
            (0b0110011, 0b111, _) => "and",
            (0b0110011, 0b110, _) => "or",
            (0b0110011, 0b100, _) => "xor",
            (0b0110011, 0b001, _) => "sll",
            (0b0110011, 0b101, 0b0000000) => "srl",
            (0b0110011, 0b101, 0b0100000) => "sra",

            (0b0010011, 0b000, _) => "addi",
            (0b0010011, 0b010, _) => "slti",
            (0b0010011, 0b111, _) => "andi",
            (0b0010011, 0b110, _) => "ori",

            (0b0000011, 0b010, _) => "lw",
            (0b0100011, 0b010, _) => "sw",

            (0b1100011, 0b000, _) => "beq",
            (0b1100011, 0b001, _) => "bne",

            (0b0110111, _, _) => "lui",
            (0b0010111, _, _) => "auipc",
            (0b1101111, _, _) => "jal",
            (0b1100111, _, _) => "jalr",

            _ => "unknown",
        }
    }
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

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let r = |r: u8| format!("x{}", r);
        let name = self.mnemonic();

        if name == "addi" && self.rd == 0 && self.rs1 == 0 && self.imm == 0 {
            return write!(f, "nop");
        }

        match self.typ {
            InstructionType::R => {
                write!(f, "{} {}, {}, {}", name, r(self.rd), r(self.rs1), r(self.rs2))
            }
            InstructionType::I => {
                write!(f, "{} {}, {}, {}", name, r(self.rd), r(self.rs1), self.imm)
            }
            InstructionType::S => {
                write!(f, "{} {}, {}({})", name, r(self.rs2), self.imm, r(self.rs1))
            }
            InstructionType::B => {
                write!(f, "{} {}, {}, {}", name, r(self.rs1), r(self.rs2), self.imm)
            }
            InstructionType::U => {
                write!(f, "{} {}, {}", name, r(self.rd), self.imm)
            }
            InstructionType::J => {
                write!(f, "{} {}, {}", name, r(self.rd), self.imm)
            }
            InstructionType::Unknown => {
                write!(
                    f,
                    "unknown(op={:#x}, f3={}, f7={}, rd={}, rs1={}, rs2={}, imm={})",
                    self.opcode, self.funct3, self.funct7, self.rd, self.rs1, self.rs2, self.imm
                )
            }
        }
    }
}

/// Sign-extend the given value from `bits` bits to 32 bits.
pub fn sign_extend(value: i32, bits: u8) -> i32 {
    let shift = 32 - bits;
    (value << shift) >> shift
}

/// A simple instruction buffer that holds raw instructions.
pub struct InstructionBuffer {
    pub queue: VecDeque<RawInstruction>,
    pub capacity: usize,
}

impl InstructionBuffer {
    /// Create a new instruction buffer with the given capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Push a raw instruction into the buffer if there is space.
    pub fn push(&mut self, instr: RawInstruction) {
        if self.queue.len() < self.capacity {
            self.queue.push_back(instr);
        }
    }

    /// Pop a batch of raw instructions from the buffer.
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
