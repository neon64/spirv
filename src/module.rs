use instruction::{InstructionHeader, InstructionWrapper};
use instruction::data::Instruction;
use enumerations::*;
use std::mem;
use std::slice;

const SPIRV_MAGIC: u32 = 0x07230203;
const SPIRV_MAGIC_REV: u32 = 0x03022307;
const SPIRV_VERSION: u32 = 99;

fn bswap_32(x: u32) -> u32 {
    ((x & 0xff000000) >> 24) | ((x & 0x00ff0000) >>  8) |
    ((x & 0x0000ff00) <<  8) | ((x & 0x000000ff) << 24)
}

#[derive(Debug)]
pub struct ModuleHeader {
    pub magic: u32,
    pub version: u32,
    pub generator: u32,
    pub bound: u32,
    pub reserved: u32
}

pub struct Module<'a> {
    pub header: &'a ModuleHeader,
    words: &'a [u32],
    start_index: usize
}

#[derive(Debug)]
pub enum SpirvError {
    NoHeader,
    WordCountIsZero,
    InvalidOpcode,
    InvalidMagicNumber(u32),
    UnsupportedVersion(u32)
}

pub struct InstructionIterator<'a> {
    words: &'a [u32],
    index: usize
}

impl<'a> Iterator for InstructionIterator<'a> {
    type Item = Result<InstructionWrapper<'a>, SpirvError>;
    fn next(&mut self) -> Option<Result<InstructionWrapper<'a>, SpirvError>> {
        let opcode_and_word_count = match self.words.get(self.index) { Some(w) => w, None => return None }; // get instruction header
        let instruction_header = InstructionHeader {
            word_count: (*opcode_and_word_count >> 16) as u16,
            opcode: unsafe { mem::transmute(*opcode_and_word_count as u16) }
        };

        // check for malformed header
        if instruction_header.word_count == 0 {
            return Some(Err(SpirvError::WordCountIsZero));
        } else if instruction_header.opcode as u16 >= Op::OpLastValue as u16 {
            return Some(Err(SpirvError::InvalidOpcode));
        }

        let instruction = {
            let body_ptr = match self.words.get(self.index + 1) { Some(w) => w, None => opcode_and_word_count } as *const _; // get the start of the instruction body
            unsafe { Instruction::from_opcode_and_ptr(instruction_header.opcode, body_ptr) }
        };

        self.index += instruction_header.word_count as usize;

        Some(Ok(InstructionWrapper {
            header: instruction_header,
            node: instruction
        }))
    }
}

impl<'a> Module<'a> {
    pub fn from_bytes(bytes: &'a Vec<u8>) -> Result<Self, SpirvError> {
        // convert the vector of bytes into a vector of words
        let words: &mut [u32] = unsafe { slice::from_raw_parts_mut(bytes.as_ptr() as *mut u32, bytes.len() / 4) };

        let mut index = 0;

        // take out the header
        let header_size = mem::size_of::<ModuleHeader>() / mem::size_of::<u32>();
        if words.len() < header_size {
            return Err(SpirvError::NoHeader);
        }

        let header = unsafe { &*(&words[0] as *const u32 as *const ModuleHeader) };
        index += header_size;

        // perform byte swapping
        if header.magic == SPIRV_MAGIC_REV {
            for word in words.iter_mut() {
                *word = bswap_32(*word);
            }
        }

        if header.magic != SPIRV_MAGIC {
            return Err(SpirvError::InvalidMagicNumber(header.magic));
        }
        if header.version != SPIRV_VERSION {
            return Err(SpirvError::UnsupportedVersion(header.version));
        }

        Ok(Module {
            header: header,
            words: words,
            start_index: index
        })
    }

    pub fn instructions(&self) -> InstructionIterator<'a> {
        InstructionIterator {
            words: self.words,
            index: self.start_index
        }
    }
}