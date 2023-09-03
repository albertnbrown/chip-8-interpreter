pub mod instruction;
use instruction::Instruction;
use std::fs;
use std::path::Path;

const MEM_SIZE: usize = 4096;
const STACK_HEIGHT: usize = 16;
const NUM_VARS: usize = 16;
const START_SLOT: usize = 0x0200;

pub struct Storage {
    // all the var size limits have custom implementations
    memory: [usize; MEM_SIZE],
    program_counter: usize,
    index_register: usize,
    stack: [usize; STACK_HEIGHT],
    variables: [usize; NUM_VARS],
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
            program_counter: 0,
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

    }

    pub fn show_memory(&self) {
        println!("{:?}", self.memory);
    }

    fn get_instruction(&mut self) -> Instruction {
        let raw_instruction: usize =
            (self.memory[self.program_counter]) << 32
            + self.memory[self.program_counter + 1];
        self.program_counter += 2;
        let instruction: Instruction = Instruction {
            identifier: (raw_instruction & 0xF000),
            x: (raw_instruction & 0x0F00),
            y: (raw_instruction & 0x00F0),
            n: (raw_instruction & 0x000F),
            nn: (raw_instruction & 0x00FF),
            nnn: raw_instruction & 0x0FFF,
        };
        return instruction;
    }
}



