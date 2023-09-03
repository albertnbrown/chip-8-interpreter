extern crate rand;

mod keyboard;

use crate::runtime::{Runtime, Instruction, display::{CHIP8_WIDTH, CHIP8_HEIGHT}, DeviceQuery};
use rand::Rng;
use std::collections::HashSet;

const VARIABLE_MODULUS: usize = 256; // the max value settable to a variable
const BIT_LENGTH: usize = 8; // the number of bits in an element of memory

// function for function map array
fn handle_error_case(_runtime: &mut Runtime, instruction: Instruction) {
    panic!("{:?} has unhandled opcode case", instruction);
}

// clear screen or pop stacked instruction
pub fn handle0(runtime: &mut Runtime, instruction: Instruction) {
    // clear screen
    if instruction.nnn == 0x0E0 {
        runtime.display.clear();
        return;
    }
    // pop stacked instruction
    if instruction.nnn == 0x0EE {
        runtime.storage.pop_pc_from_stack();
        return;
    }
    if instruction.nnn == 0 {
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
fn handle8XY0(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 0);
    runtime.storage.variables[instruction.x] = runtime.storage.variables[instruction.y];
}

// set vx to vx | vy
fn handle8XY1(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 1);
    runtime.storage.variables[instruction.x] |= runtime.storage.variables[instruction.y];
}

// set vx to vx & vy
fn handle8XY2(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 2);
    runtime.storage.variables[instruction.x] &= runtime.storage.variables[instruction.y];
}

// set vx to vx ^ vy
fn handle8XY3(runtime: &mut Runtime, instruction: Instruction) {
    assert!(instruction.n == 3);
    runtime.storage.variables[instruction.x] ^= runtime.storage.variables[instruction.y];
}

// set vx to vx + vy with carry on overflow
fn handle8XY4(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[instruction.x] += runtime.storage.variables[instruction.y];
    runtime.storage.variables[0x0F] = if runtime.storage.variables[instruction.x] > VARIABLE_MODULUS { 1 } else { 0 };
    runtime.storage.variables[instruction.x] %= VARIABLE_MODULUS;
}

// set vx to vx - vy with carry on LACK of underflow
fn handle8XY5(runtime: &mut Runtime, instruction: Instruction) {
    let carry: usize = if runtime.storage.variables[instruction.y] < runtime.storage.variables[instruction.x] { 1 } else { 0 };
    runtime.storage.variables[instruction.x] = if carry == 1 {
        runtime.storage.variables[instruction.x] - runtime.storage.variables[instruction.y]
    } else {
        runtime.storage.variables[instruction.x] + VARIABLE_MODULUS - runtime.storage.variables[instruction.y]
    };
    runtime.storage.variables[0x0F] = carry;
}

// right shift vx with carry for underflow
fn handle8XY6(runtime: &mut Runtime, instruction: Instruction) {
    let carry = runtime.storage.variables[instruction.x] & 1; // grab lowest bit that'll be shifted out
    runtime.storage.variables[instruction.x] >>= 1;
    runtime.storage.variables[0x0F] = carry;
}

// set vx to vy - vx with carry on LACK of underflow
fn handle8XY7(runtime: &mut Runtime, instruction: Instruction) {
    let carry = if runtime.storage.variables[instruction.x] < runtime.storage.variables[instruction.y] { 1 } else { 0 };
    runtime.storage.variables[instruction.x] = if carry == 1 {
        runtime.storage.variables[instruction.y] - runtime.storage.variables[instruction.x]
    } else {
        runtime.storage.variables[instruction.y] + VARIABLE_MODULUS - runtime.storage.variables[instruction.x]
    };
    runtime.storage.variables[0x0F] = carry;
}

// left shift vx with carry for overflow
fn handle8XYE(runtime: &mut Runtime, instruction: Instruction) {
    let carry = (runtime.storage.variables[instruction.x] >> (BIT_LENGTH - 1)) & 1; // grab highest bit that'll be shifted out
    // quirk dependent: runtime.storage.variables[instruction.x] = runtime.storage.variables[instruction.y];
    runtime.storage.variables[instruction.x] <<= 1;
    runtime.storage.variables[instruction.x] %= VARIABLE_MODULUS;
    runtime.storage.variables[0x0F] = carry;
}

// branching for several 8 opcode cases
pub fn handle8(runtime: &mut Runtime, instruction: Instruction) {
    match instruction.n {
        0x0 => handle8XY0(runtime, instruction),
        0x1 => handle8XY1(runtime, instruction),
        0x2 => handle8XY2(runtime, instruction),
        0x3 => handle8XY3(runtime, instruction),
        0x4 => handle8XY4(runtime, instruction),
        0x5 => handle8XY5(runtime, instruction),
        0x6 => handle8XY6(runtime, instruction),
        0x7 => handle8XY7(runtime, instruction),
        0xE => handle8XYE(runtime, instruction),
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

// skip if key pressed/notpressed
pub fn handleE(runtime: &mut Runtime, instruction: Instruction) {
    let target_key = keyboard::KEY_MAP[runtime.storage.variables[instruction.x]];
    let keys = runtime.device_state.get_keys();
    if instruction.nn == 0x9E {
        if keys.contains(&target_key) {
            runtime.storage.program_counter += 2;
        }
        return;
    }
    if instruction.nn == 0xA1 {
        if !keys.contains(&(target_key as u16)) {
            runtime.storage.program_counter += 2;
        }
        return;
    }
    panic!("bad E opcode nn: {:?}", instruction);
}

// set vx to the delay timer value
fn handleFX07(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.variables[instruction.x] = runtime.delay_timer;
}

// get keypress to vx
fn handleFX0A(runtime: &mut Runtime, instruction: Instruction) {
    let pressed_keys = runtime.device_state.get_keys();
    let pressed_key_set = pressed_keys.iter().collect::<HashSet<_>>();
    let key_set_ref = keyboard::KEY_MAP.iter().collect::<HashSet<_>>();
    let pressed_device_keys: Vec<&&u16> = pressed_key_set.intersection(&key_set_ref).collect();
    
    if runtime.current_key_press.is_some() && !pressed_device_keys.contains(&&&runtime.current_key_press.unwrap()) {
        let key_value = keyboard::KEY_MAP.iter().position(|e| *e==runtime.current_key_press.unwrap()).unwrap();
        runtime.storage.variables[instruction.x] = key_value;
        runtime.current_key_press = None;
        return;
    }
    if runtime.current_key_press.is_none() && pressed_device_keys.len() != 0 {
        let found_key = **pressed_device_keys[0];
        runtime.current_key_press = Some(found_key);
    }
    runtime.storage.program_counter -= 2;
}

// set the delay timer value to vx
fn handleFX15(runtime: &mut Runtime, instruction: Instruction) {
    runtime.delay_timer = runtime.storage.variables[instruction.x];
}

// set the sound timer value to vx
fn handleFX18(runtime: &mut Runtime, instruction: Instruction) {
    runtime.sound_timer = runtime.storage.variables[instruction.x];
}

// add vx it index register
fn handleFX1E(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.index_register += runtime.storage.variables[instruction.x];
    if runtime.storage.index_register > 0x0FFF {
        runtime.storage.variables[0x0F] = 1;
    }
}

// set index register to font of vx
fn handleFX29(runtime: &mut Runtime, instruction: Instruction) {
    runtime.storage.index_register = runtime.storage.get_font_item_location(runtime.storage.variables[instruction.x]);
}

// decimal conversion of vx into memory starting with the index register
fn handleFX33(runtime: &mut Runtime, instruction: Instruction) {
    let vx = runtime.storage.variables[instruction.x];
    let hundreds = (vx / 100) % 10;
    let tens = (vx / 10) % 10;
    let ones = vx % 10;
    let start_address = runtime.storage.index_register;
    runtime.storage.memory[start_address] = hundreds;
    runtime.storage.memory[start_address + 1] = tens;
    runtime.storage.memory[start_address + 2] = ones;
}

// store v0 to vx into memory starting with the index register
fn handleFX55(runtime: &mut Runtime, instruction: Instruction) {
    let start_address = runtime.storage.index_register;
    for i in 0..(instruction.x+1) {
        runtime.storage.memory[start_address + i] = runtime.storage.variables[i];
    }
}

// load v0 to vx from memory starting with the index register
fn handleFX65(runtime: &mut Runtime, instruction: Instruction) {
    let start_address = runtime.storage.index_register;
    for i in 0..(instruction.x+1) {
        runtime.storage.variables[i] = runtime.storage.memory[start_address + i];
    }
}

// grab bag opcodes
pub fn handleF(runtime: &mut Runtime, instruction: Instruction) {
    match instruction.nn {
        0x07 => handleFX07(runtime, instruction),
        0x0A => handleFX0A(runtime, instruction),
        0x15 => handleFX15(runtime, instruction),
        0x18 => handleFX18(runtime, instruction),
        0x1E => handleFX1E(runtime, instruction),
        0x29 => handleFX29(runtime, instruction),
        0x33 => handleFX33(runtime, instruction),
        0x55 => handleFX55(runtime, instruction),
        0x65 => handleFX65(runtime, instruction),
        _ => handle_error_case(runtime, instruction),
    }
}
