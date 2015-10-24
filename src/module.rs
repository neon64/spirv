use instruction::InstructionHeader;
use instruction::data::Instruction;
use spirv::*;
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
struct ModuleHeader {
    magic: u32,
    version: u32,
    generator: u32,
    bound: u32,
    reserved: u32
}

pub struct Module<'a> {
    header: &'a ModuleHeader,
    instruction_iterator: InstructionIterator<'a>
    /*start: *const InstructionHeader,
    end: *const InstructionHeader,*/

    // The module layout
    //source: Option<OpSourceStruct>                 // optional / single
    /*OP_STRUCT(OpSourceExtension) const* sourceExtensions;       // optional / multiple
    OP_STRUCT(OpCompileFlag) const*     compileFlags;           // optional / multiple
    OP_STRUCT(OpExtension) const*       extensions;             // optional / multiple
    OP_STRUCT(OpExtInstImport) const*   extInstImport;          // optional / multiple
    OP_STRUCT(OpMemoryModel) const*     memoryModel;            // required / single
    OP_STRUCT(OpEntryPoint) const*      entryPoints;            // required / multiple
    OP_STRUCT(OpExecutionMode) const*   executionModes;         // required / multiple

    // all debug and annotation instructions
    OP_STRUCT(OpString) const*          strings;                // all string declarations,     optional / multiple
    InstructionHeader const*            names;                  // all name declarations,       optional / multiple
    OP_STRUCT(OpLine) const*            lines;                  // all line declarations,       optional / multiple
    InstructionHeader const*            decorations;            // all decoration declarations, optional / multiple

    // all type declaration / all constant instructions
    InstructionHeader const*            declarations;
    OP_STRUCT(OpFunction) const*        functionDeclarations;   // optional / multiple
    OP_STRUCT(OpFunction) const*        functionDefinitions;    // required / multiple*/
}

#[derive(Debug)]
pub enum SpirvError {
    NoHeader,
    WordCountIsZero,
    InvalidOpcode,
    InvalidMagicNumber(u32),
    UnsupportedVersion(u32)
}

struct InstructionIterator<'a> {
    words: &'a [u32],
    index: usize
}

impl<'a> InstructionIterator<'a> {
    pub fn next(&mut self) -> Option<Result<Instruction<'a>, SpirvError>> {
        let opcode_and_word_count = match self.words.get(self.index) { Some(w) => w, None => return None }; // get instruction header
        //println!("{} => Raw Opcode and Word Count: {:#x}", self.index, opcode_and_word_count);
        let instruction_header = InstructionHeader {
            word_count: (*opcode_and_word_count >> 16) as u16,
            opcode: unsafe { mem::transmute(*opcode_and_word_count as u16) }
        };
        /*println!("{} => Opcode No: {}", self.index, *opcode_and_word_count as u16);
        println!("{} => Header: {:?}", self.index, instruction_header);*/

        // check for malformed header
        if instruction_header.word_count == 0 {
            return Some(Err(SpirvError::WordCountIsZero));
        } else if instruction_header.opcode as u16 >= Op::OpLastValue as u16 {
            return Some(Err(SpirvError::InvalidOpcode));
        }

        let instruction = {
            let body_ptr = match self.words.get(self.index + 1) { Some(w) => w, None => return None } as *const _; // get the start of the instruction body
            //println!("    Start: {}", unsafe { *body_ptr });
            unsafe { Instruction::from_opcode_and_ptr(instruction_header.opcode, body_ptr) }
        };
        self.index += instruction_header.word_count as usize;

        Some(Ok(instruction))
    }
}

/*impl<'a> Iterator for InstructionIterator<'a> {
    type Item = Result<Instruction<'a>, SpirvError>;
    fn next(&mut self) -> Option<Self::Item> {
        let opcode_and_word_count = match self.words.get(self.index) { Some(w) => w, None => return None }; // get instruction header
        println!("{} => Raw Opcode and Word Count: {:#x}", self.index, opcode_and_word_count);
        let instruction_header = InstructionHeader {
            word_count: (*opcode_and_word_count >> 16) as u16,
            opcode: unsafe { mem::transmute(*opcode_and_word_count as u16) }
        };
        println!("{} => Opcode No: {}", self.index, *opcode_and_word_count as u16);
        println!("{} => Header: {:?}", self.index, instruction_header);

        // check for malformed header
        if instruction_header.word_count == 0 {
            return Some(Err(SpirvError::WordCountIsZero));
        } else if instruction_header.opcode as u16 >= Op::OpLastValue as u16 {
            return Some(Err(SpirvError::InvalidOpcode));
        }

        let instruction = {
            let body_ptr = match self.words.get(self.index + 1) { Some(w) => w, None => return None } as *const _; // get the start of the instruction body
            println!("    Start: {}", unsafe { *body_ptr });
            unsafe { Instruction::from_opcode_and_ptr(instruction_header.opcode, body_ptr) }
        };
        self.index += instruction_header.word_count as usize;

        Some(Ok(instruction))
    }
}*/

impl<'a> Module<'a> {
    pub fn from_raw(bytes: &'a Vec<u8>) -> Result<Self, SpirvError> {
        // convert the vector of bytes into a vector of words
        let words: &mut [u32] = unsafe { slice::from_raw_parts_mut(bytes.as_ptr() as *mut u32, bytes.len() / 4) };

        let mut index = 0;

        // take out the header
        let header_size = mem::size_of::<ModuleHeader>() / mem::size_of::<u32>();
        if words.len() < header_size {
            return Err(SpirvError::NoHeader);
        }

        let header = unsafe { &*(&words[0] as *const u32 as *const ModuleHeader) };
        println!("Header: {:?}", header);
        index += header_size;

        // perform byte swapping
        if header.magic == SPIRV_MAGIC_REV {
            println!("swapping bytes");
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
            instruction_iterator: InstructionIterator {
                words: words,
                index: index
            }
        })
    }

    pub fn instructions(&mut self) -> &mut InstructionIterator<'a> {
        &mut self.instruction_iterator
    }
}