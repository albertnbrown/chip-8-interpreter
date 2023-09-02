pub mod instruction;
use instruction::Instruction;
use std::fs;

const MEM_SIZE: usize = 4096;
const STACK_HEIGHT: usize = 16;
const NUM_VARS: usize = 16;
const START_SLOT: usize = 0x0200;

pub struct Storage {
    memory: [u8; MEM_SIZE],
    program_counter: u16, // u12 max for memory on these u16 vars
    index_register: u16,
    stack: [u16; STACK_HEIGHT],
    variables: [u8; NUM_VARS],
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
        let contents: Vec<u8> = fs::read(
            format!("../../../programs/{file_name}"),
        ).expect(
            "Should have been able to read the file",
        );
        let end_slot: usize = START_SLOT + contents.len();
        assert!(end_slot <= MEM_SIZE, "program out of bounds");
        let program_slice: &mut [u8] = &mut self.memory[START_SLOT .. end_slot];
        program_slice.iter_mut().enumerate().for_each(|(index, slot)| {
            *slot = contents[index];
        });
    }

    fn load_font(&mut self) {
        
    }

    fn get_instruction(&mut self) -> Instruction {
        let raw_instruction: u16 = 
            (self.memory[self.program_counter as usize] as u16) << 32
            + self.memory[self.program_counter as usize + 1] as u16;
        self.program_counter += 2;
        let instruction: Instruction = Instruction {
            identifier: (raw_instruction & 0xF000) as u8,
            x: (raw_instruction & 0x0F00) as u8,
            y: (raw_instruction & 0x00F0) as u8,
            n: (raw_instruction & 0x000F) as u8,
            nn: (raw_instruction & 0x00FF) as u8,
            nnn: raw_instruction & 0x0FFF,
        };
        return instruction;
    }
}



