extern crate rand;

use crate::runtime::{Runtime, Instruction, display::{CHIP8_WIDTH, CHIP8_HEIGHT}};
use rand::Rng;

const VARIABLE_MODULUS: usize = 256; // the max value settable to a variable
const BIT_LENGTH: usize = 8; // the number of bits in an element of memory
const BYTE_SIZE: usize = 16;

// function for function map array
fn handle_error_case(_runtime: &mut Runtime, instruction: Instruction) {
    panic!("{:?} has unhandled opcode case", instruction);
}

// clear screen or pop stacked instruction
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
    if (instruction.nnn == 0) {
        panic!("escaped program in memory");
    }
    panic!("unsupported program with 0NNN: Execute machine language routine opcode");
}

// jump to nnn
pub fn handle1(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.program_counter = instruction.nnn;
}

// jump to nnn and add current PC to stack
pub fn handle2(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.add_pc_to_stack();
    runtime.storage.program_counter = instruction.nnn;
}

// skip if vx == nn
pub fn handle3(runtime: &mut Runtime, instruction: Instruction) {
    if runtime.storage.variables[instruction.x] == instruction.nn {
        runtime.storage.program_counter += 2;
    }
}

// skip if vx != nn
pub fn handle4(runtime: &mut Runtime, instruction: Instruction) {
    if runtime.storage.variables[instruction.x] != instruction.nn {
        runtime.storage.program_counter += 2;
    }
}

// skip if vx == vy
pub fn handle5(runtime: &mut Runtime, instruction: Instruction) {
    if runtime.storage.variables[instruction.x] == runtime.storage.variables[instruction.y] {
        runtime.storage.program_counter += 2;
    }
}

// set variable register
pub fn handle6(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[instruction.x] = instruction.nn;
}

// add to variable register
pub fn handle7(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[instruction.x] += instruction.nn;
    runtime.storage.variables[instruction.x] %= VARIABLE_MODULUS;
}

// set vx to vy
fn handle80(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 0);
    runtime.storage.variables[instruction.x] = runtime.storage.variables[instruction.y];
}

// set vx to vx | vy
fn handle81(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 1);
    runtime.storage.variables[instruction.x] |= runtime.storage.variables[instruction.y];
}

// set vx to vx & vy
fn handle82(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 2);
    runtime.storage.variables[instruction.x] &= runtime.storage.variables[instruction.y];
}

// set vx to vx ^ vy
fn handle83(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 3);
    runtime.storage.variables[instruction.x] ^= runtime.storage.variables[instruction.y];
}

// set vx to vx + vy with carry on overflow
fn handle84(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[instruction.x] += runtime.storage.variables[instruction.y];
    runtime.storage.variables[0x0F] = if runtime.storage.variables[instruction.x] > VARIABLE_MODULUS { 1 } else { 0 };
    runtime.storage.variables[instruction.x] %= VARIABLE_MODULUS;
}

// set vx to vx - vy with carry on LACK of underflow
fn handle85(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[0x0F] = if runtime.storage.variables[instruction.x] < runtime.storage.variables[instruction.y] { 1 } else { 0 };
    runtime.storage.variables[instruction.x] = if runtime.storage.variables[0x0F] == 1 {
        runtime.storage.variables[instruction.x] + VARIABLE_MODULUS - runtime.storage.variables[instruction.y]
    } else {
        runtime.storage.variables[instruction.x] - runtime.storage.variables[instruction.y]
    }
}

// right shift vx with carry for underflow
fn handle86(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[0x0F] = runtime.storage.variables[instruction.x] & 1; // grab lowest bit that'll be shifted out
    runtime.storage.variables[instruction.x] >>= 1;
}

// set vx to vy - vx with carry on LACK of underflow
fn handle87(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[0x0F] = if runtime.storage.variables[instruction.y] < runtime.storage.variables[instruction.x] { 1 } else { 0 };
    runtime.storage.variables[instruction.x] = if runtime.storage.variables[0x0F] == 1 {
        runtime.storage.variables[instruction.y] + VARIABLE_MODULUS - runtime.storage.variables[instruction.x]
    } else {
        runtime.storage.variables[instruction.y] - runtime.storage.variables[instruction.x]
    }
}

// left shift vx with carry for overflow
fn handle8E(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[0x0F] = (runtime.storage.variables[instruction.x] >> (BYTE_SIZE - 1)) & 1; // grab highest bit that'll be shifted out
    runtime.storage.variables[instruction.x] <<= 1;
    runtime.storage.variables[instruction.x] %= VARIABLE_MODULUS;
}

// branching for several 8 opcode cases
pub fn handle8(runtime: &mut Runtime, instruction: Instruction) {
    match instruction.n {
        0x0 => handle80(runtime, instruction),
        0x1 => handle81(runtime, instruction),
        0x2 => handle82(runtime, instruction),
        0x3 => handle83(runtime, instruction),
        0x4 => handle84(runtime, instruction),
        0x5 => handle85(runtime, instruction),
        0x6 => handle86(runtime, instruction),
        0x7 => handle87(runtime, instruction),
        0xE => handle8E(runtime, instruction),
        _ => handle_error_case(runtime, instruction),
    }
}

// skip if vx != vy
pub fn handle9(runtime: &mut Runtime, instruction: Instruction) {
    if runtime.storage.variables[instruction.x] != runtime.storage.variables[instruction.y] {
        runtime.storage.program_counter += 2;
    }
}

// set index register
pub fn handleA(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.index_register = instruction.nnn;
}

// jump to nnn + v0
pub fn handleB(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.program_counter = instruction.nnn + runtime.storage.variables[0];
}

// set vx to nn & a random number
pub fn handleC(runtime: &mut Runtime, instruction: Instruction) {
    let random: u16 = rand::thread_rng().gen();
    runtime.storage.variables[instruction.x] = (random as usize) & instruction.nn;
}


// draw sprites to screen
pub fn handleD(runtime: &mut Runtime, instruction: Instruction) {
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

// grab bag opcodes
pub fn handleF(runtime: &mut Runtime, instruction: Instruction) {
    match instruction.nn {
        _ => handle_error_case(runtime, instruction),
    }
}