pub mod instruction;
use instruction::Instruction;

pub struct Storage {
    memory: [u8; 4096],
    program_counter: u16, // u12 max for memory on these u16 vars
    index_register: u16,
    stack: [u16; 16],
    variables: [u8; 16],
}

impl Storage {
    fn load_program(&mut self, file_name: String) {
        //this function will take the filename, find it, and load it into memory
    }
    fn get_instruction(&mut self) -> Instruction {
        let raw_instruction: u16 = (self.memory[self.program_counter as usize] as u16) << 32 + self.memory[self.program_counter as usize + 1] as u16;
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



