pub mod storage;
pub mod display;

pub use storage::{instruction, Storage};
use instruction::Instruction;

pub struct Runtime {
    storage: Storage,
}

impl Runtime {
    pub fn initialize(&mut self, file_name: String) -> Runtime {
        return Runtime {storage: Storage::initialize(file_name)};
    }
    // fn parse_instruction(&mut self, instruction: Instruction) -> Operation  {

    // }
}

