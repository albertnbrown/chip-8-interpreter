pub mod storage;
pub mod display;
mod operators;

use storage::{instruction::Instruction, Storage};
use operators::*;
use display::Display;

const OPCODE_INITIAL_CASES: usize = 16;

pub struct Runtime {
    storage: Storage,
    opcode_handlers: [fn(&mut Storage, &mut Display, Instruction);16],
}

impl Runtime {
    pub fn initialize(&mut self, file_name: String) -> Runtime {
        let opcode_handlers: [fn(&mut Storage, &mut Display, Instruction); OPCODE_INITIAL_CASES] = [
            handle0,
            handle1,
            handle2,
            handle3,
            handle4,
            handle5,
            handle6,
            handle7,
            handle8,
            handle9,
            handleA,
            handleB,
            handleC,
            handleD,
            handleE,
            handleF,
        ];
        let storage: Storage = Storage::initialize(file_name);
        return Runtime {storage,  opcode_handlers};
    }
    // fn parse_instruction(&mut self, instruction: Instruction) -> Operation  {

    // }
}

