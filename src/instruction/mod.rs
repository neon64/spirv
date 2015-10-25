use std::ffi::CStr;
use std::fmt;
use std::str;
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
pub struct LiteralString(char);

impl LiteralString {
    fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(unsafe { CStr::from_ptr(self as *const _ as *const i8) }.to_bytes())
    }
}


impl fmt::Debug for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str().unwrap_or("<invalid string>"))
    }
}