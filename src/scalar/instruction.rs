#[derive(Copy, Clone)]
pub struct RawInstruction {
    pub data: u32
}

pub enum InstructionType {

}

pub struct Instruction {
    pub typ: InstructionType,
}
