extern crate spirv;

use std::fs::File;
use std::io::Read;
use spirv::Module;

#[test]
fn it_parses_spirv_files() {
    let mut file = File::open("tests/test_shader.spv").unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    let module = Module::from_bytes(&bytes).unwrap();
    for instruction in module.instructions() {
        println!("{:?}", instruction.unwrap().node);
    }
}