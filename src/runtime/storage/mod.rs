pub mod instruction;
mod font;
use instruction::Instruction;
use font::FONT;
use std::fs;
use std::path::Path;

const MEM_SIZE: usize = 4096;
const STACK_HEIGHT: usize = 16;
const NUM_VARS: usize = 16;
const START_SLOT: usize = 0x0200;
const FONT_START: usize = 0x0050;
const BYTE_LENGTH: usize = 8;
const HALF_BYTE: usize = 4;

pub struct Storage {
    // all the var size limits have custom implementations
    pub memory: [usize; MEM_SIZE],
    pub program_counter: usize,
    pub index_register: usize,
    pub stack: [usize; STACK_HEIGHT],
    pub variables: [usize; NUM_VARS],
}

/*
can do this eventually:
impl Default for Point {
  fn default() -> Self {
    Point { x: 0, y: 0 }
  }
}

fn main() {
  let p1 = Point { x: 1, ..Default::default() };

  println!("{:?}", p1); // Point { x: 1, y: 0 }
}
 */

impl Storage {
    pub fn initialize(file_name: String) -> Storage{
        let mut storage: Storage = Storage {
            memory: [0; MEM_SIZE],
            program_counter: START_SLOT, // start of the program
            index_register: 0,
            stack: [0; STACK_HEIGHT],
            variables: [0; NUM_VARS],
        };
        storage.load_font();
        storage.load_program(file_name);

        return storage;
    }

    fn load_program(&mut self, file_name: String) {
        let string_path = format!("./programs/{}", file_name);
        let filepath = Path::new(&string_path);
        assert!(filepath.exists(), "{:#?}", filepath.display());

        let contents: Vec<u8> = fs::read(
            filepath,
        ).expect(
            &format!("path {} to file not found", filepath.display()),
        );

        let end_slot: usize = START_SLOT + contents.len();
        assert!(end_slot <= MEM_SIZE, "program out of bounds");

        let program_slice: &mut [usize] = &mut self.memory[START_SLOT .. end_slot];
        program_slice.iter_mut().enumerate().for_each(|(index, slot)| {
            *slot = contents[index] as usize;
        });
    }

    fn load_font(&mut self) {
        let font_slice: &mut [usize] = &mut self.memory[FONT_START .. FONT_START + FONT.len()];
        font_slice.iter_mut().enumerate().for_each(|(index, slot)| {
            *slot = FONT[index];
        });
    }

    pub fn show_memory(&self) {
        println!("{:?}", self.memory);
    }

    pub fn pop_pc_from_stack(&mut self) {
        let maybe_last_stack_element_pos = self.stack.iter().rev().position(|&x| x != 0);
        let last_stack_element_pos = maybe_last_stack_element_pos.unwrap();
        let last_stack_element = self.stack[last_stack_element_pos];
        self.stack[last_stack_element_pos] = 0;
        self.program_counter = last_stack_element;
    }

    pub fn get_instruction(&mut self) -> Instruction {
        // grab the two bytes starting at PC and collate them
        let raw_instruction: usize =
            (self.memory[self.program_counter] << BYTE_LENGTH)
            + self.memory[self.program_counter + 1];
        self.program_counter += 2;
        let instruction: Instruction = Instruction {
            identifier: (raw_instruction & 0xF000) >> (BYTE_LENGTH + HALF_BYTE),
            x: (raw_instruction & 0x0F00) >> BYTE_LENGTH,
            y: (raw_instruction & 0x00F0) >> HALF_BYTE,
            n: (raw_instruction & 0x000F),
            nn: (raw_instruction & 0x00FF),
            nnn: raw_instruction & 0x0FFF,
        };
        return instruction;
    }
}



