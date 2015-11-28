use std::ffi::CStr;
use std::fmt;
use std::mem;
use std::str;
use std::ops::Index;
use instruction::data::Instruction;
use enumerations::Op;

pub mod data;

// Header of all instructions, contains the opcode identifier and the size of the instruction
#[derive(Debug)]
pub struct InstructionHeader {
    pub word_count: u16,
    pub opcode: Op

    // TODO: figure out how to best validate an instruction using it's opcode
    //      and the definitions below
}

pub struct InstructionWrapper<'a> {
    pub header: InstructionHeader,
    pub node: Instruction<'a>
}

impl<'a> InstructionWrapper<'a> {
    pub fn len(&self) -> usize {
        self.header.word_count as usize
    }
}

pub type Id = u32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LiteralNumber(u32);

impl LiteralNumber {
    pub fn as_u32(&self) -> u32 {
        self.0
    }
    pub fn as_i32(&self) -> i32 {
        unsafe { mem::transmute(self.0) }
    }
    pub fn as_f32(&self) -> f32 {
        unsafe { mem::transmute(self.0) }
    }
}

pub struct LiteralString(char);

impl LiteralString {
    pub fn as_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(unsafe { CStr::from_ptr(self as *const _ as *const i8) }.to_bytes())
    }
}


impl fmt::Debug for LiteralString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str().unwrap_or("<invalid string>"))
    }
}

#[derive(Debug)]
pub struct UnsizedArray<T: fmt::Debug> {
    first_item: T
}

impl<T: fmt::Debug> Index<usize> for UnsizedArray<T> {
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a T {
        unsafe { &*(self as *const UnsizedArray<T> as *const _).offset(index as isize) }
    }
}