use crate::runtime::{Runtime, Instruction, display::{CHIP8_WIDTH, CHIP8_HEIGHT}};
use super::{storage, display, audio};

const VARIABLE_MODULUS: usize = 256; // the max value settable to a variable
const BIT_LENGTH: usize = 8; // the number of bits in an element of memory

pub fn handle0(runtime: &mut Runtime, instruction: Instruction) {
    // clear screen
    if (instruction.nnn == 0x0E0) {
        runtime.display.clear();
        return;
    }
    // pop stacked instruction
    if (instruction.nnn == 0x0EE) {
        runtime.storage.pop_pc_from_stack();
        return;
    }
    panic!("unsupported program with 0NNN: Execute machine language routine opcode");
}

pub fn handle1(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.program_counter = instruction.nnn;
}

pub fn handle2(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handle3(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handle4(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handle5(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handle6(runtime: &mut Runtime, instruction: Instruction) {
    // set variable register
    runtime.storage.variables[instruction.x] = instruction.nn;
}

pub fn handle7(runtime: &mut Runtime, instruction: Instruction) {
    // add to variable register
    runtime.storage.variables[instruction.x] += instruction.nn;
    runtime.storage.variables[instruction.x] %= VARIABLE_MODULUS;
}

pub fn handle8(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handle9(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handleA(runtime: &mut Runtime, instruction: Instruction) {
    // set index register
    runtime.storage.index_register = instruction.nnn;
}

pub fn handleB(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handleC(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handleD(runtime: &mut Runtime, instruction: Instruction) {
    // draw to screen
    let vx = runtime.storage.variables[instruction.x] % CHIP8_WIDTH;
    let vy = runtime.storage.variables[instruction.y] % CHIP8_HEIGHT;
    runtime.storage.variables[0x0F] = 0;
    let index = runtime.storage.index_register;

    let mut new_flips: [[bool; CHIP8_WIDTH]; CHIP8_HEIGHT] = [[false; CHIP8_WIDTH]; CHIP8_HEIGHT];

    // only do rows that stay on the screen
    let imax: usize = if vy + instruction.n > CHIP8_HEIGHT {
        CHIP8_HEIGHT - vy
    } else {
        instruction.n
    };

    // only draw the sprite in the amount of the row left on the screen
    let jmax: usize = if vx + BIT_LENGTH > CHIP8_WIDTH {
        CHIP8_WIDTH - vx
    } else {
        BIT_LENGTH
    };

    for i in 0..imax {
        let sprite = runtime.storage.memory[index + i];

        for j in 0..jmax {
            new_flips[vy + i][vx + j] = sprite & (1 << (BIT_LENGTH - 1 - j)) > 0;
        }
    }

    if runtime.display.draw(new_flips) {
        runtime.storage.variables[0x0F] = 1;
    }
}

pub fn handleE(runtime: &mut Runtime, instruction: Instruction) {

}

pub fn handleF(runtime: &mut Runtime, instruction: Instruction) {

}