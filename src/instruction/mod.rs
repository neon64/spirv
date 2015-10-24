use spirv::Op;

pub mod data;

// Header of all instructions, contains the opcode identifier and the size of the instruction
#[derive(Debug)]
pub struct InstructionHeader {
    pub word_count: u16,
    pub opcode: Op

    // TODO: figure out how to best validate an instruction using it's opcode
    //      and the definitions below
}

pub type Id = u32;
pub type LiteralNumber = u32;
pub type LiteralString = [char; 1];