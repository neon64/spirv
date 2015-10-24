mod instruction;
mod module;
mod spirv;
use std::io::Read;
use std::fs::File;
use module::Module;

fn main() {
    println!("{:?}", "hi");
    let mut file = File::open("src/test_shader.spv").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let mut module = Module::from_raw(&bytes).unwrap();
    let iterator = module.instructions();
    while let Some(instruction) = iterator.next() {
        //println!("{:?}", instruction.unwrap());
    }
}