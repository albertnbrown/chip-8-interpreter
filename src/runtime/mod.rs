extern crate sdl2;

mod storage;
mod display;
mod audio;
mod operators;

use storage::{instruction::Instruction, Storage};
use operators::*;
use display::Display;
use audio::Audio;

use std::time::{Duration, Instant};
use std::thread::sleep;
use std::io::{stdin, stdout, Read, Write};

const OPCODE_INITIAL_CASES: usize = 16;
const CALC_PER_FRAME: usize = 12;
const FRAME_TIME: u32 = 16666666; // in nanos
const DEBUG: bool = false;


fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
pub struct Runtime {
    pub storage: Storage,
    pub display: Display,
    pub audio: Audio,
    opcode_handlers: [fn(&mut Runtime, Instruction); OPCODE_INITIAL_CASES],
    pub delay_timer: usize,
    pub sound_timer: usize,
}

impl Runtime {
    pub fn initialize(file_name: String) -> Runtime {
        let sdl_context = sdl2::init().unwrap();
        let opcode_handlers: [fn(&mut Runtime, Instruction); OPCODE_INITIAL_CASES] = [
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
        let display: Display = Display::initialize(&sdl_context);
        let audio: Audio = Audio::initialize(&sdl_context);
        return Runtime {
            storage,  display, audio, opcode_handlers, delay_timer: 0, sound_timer: 0};
    }
    
    pub fn frame(&mut self) {
        let start: Instant = Instant::now();
        
        for _i in 0..CALC_PER_FRAME {
            let instruction: Instruction = self.storage.get_instruction();
            if DEBUG { pause(); println!("{:?}", instruction); }
            self.opcode_handlers[instruction.identifier](self, instruction); 
        }
            
        if (self.sound_timer > 0) {
            self.audio.start_beep();
            self.sound_timer -= 1;
        } else {
            self.audio.stop_beep();
        }
        if (self.delay_timer > 0) {
            self.delay_timer -= 1;
        }
        let calculation_time: Duration = Instant::now().duration_since(start);
        // we can assume that our calculations won't take nearly enough time for this duration to ever underflow
        if calculation_time.subsec_nanos() < FRAME_TIME {
            sleep(Duration::new(0, FRAME_TIME - calculation_time.subsec_nanos()));
        }
    }
}

