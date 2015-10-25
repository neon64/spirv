# SPIR-V Parser for Rust

This project contains an experimental SPIR-V parser.
Once provided with a vector of bytes, it constructs a read-only in-memory representation of the SPIR-V module.

The instruction definitions have been ported from [Logical Error's SPIR-V parser](https://github.com/LogicalError/spir-v-parser).
However it uses the enumeration of Opcodes is taken from [this experimental parser](https://github.com/kusma/SPIR-V). These don't quite match the opcodes in the specification (as of Version 0.99 revision 31) but they are compatible with the SPIR-V outputted by `glslangValidator`.

## Usage

At its most basic level, SPIR-V is parsed using the `spirv::Module::from_bytes()` method. This takes in a reference to a `Vec<u8>` which should contain the SPIR-V code, and returns the `Module` struct. Calling `module.instructions()` on the module will return an iterator over the instructions in the module.

    extern crate spirv;

    use std::io::Read;
    use std::fs::File;
    use spirv::{Module, Instruction};

    fn main() {
        let mut file = File::open("your_shader_file.spv").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let mut module = Module::from_bytes(&bytes).unwrap();
        for instruction in module.instructions() {
            let inst = instruction.unwrap();
            println!("{:?}", inst);
        }
    }

At the moment this basic parser hangs on my computer because the `spirv::Instruction` enum is too big to display.

## License

Please do anything you want with this code. It would be great though if you could let me know if you're making something interesting with it!
Issues and pull requests most certainly welcome!