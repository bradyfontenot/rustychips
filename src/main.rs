
extern crate sdl2; 

use std::env;
use std::{thread, time};
use std::time::Duration;

mod chip8;
mod cpu;
mod display;
mod input;

fn main() {
    let mut timer_counter: u128 = 0;
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    println!("The ROM is: {}", filename);

    // initialize screen
    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Display::new(&sdl_context);
    // let mut event_pump = sdl_context.event_pump().unwrap();

    // initialize chip8 state
    let mut chip8 = chip8::Chip8::init();
    let mut input = input::Input::init(&sdl_context);
    // load rom
    match cpu::load_rom(filename, &mut chip8) {
        Ok(_) => println!("Woah"),
        Err(_) => println!("Nope")
    };
    
    loop {
        display.draw_canvas();
        if input.handle_key_event() == 0 {
            break
        }

        cpu::execute(cpu::read_opcode(&mut chip8), &mut chip8, &mut display, &mut input);
        // kp = 0;
        timer_counter += 1;
        if timer_counter == 16666666{
            if chip8.delay_timer() != 0 {
                chip8.set_delay_timer(chip8.delay_timer() - 1);
            }
            if chip8.sound_timer() != 0{
                chip8.set_sound_timer(chip8.sound_timer() - 1);
            }
            timer_counter = 0;
        }
        // thread::sleep(time::Duration::from_millis(30));
        // thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }


    // println!("opcode? {:x}", opcode);

}