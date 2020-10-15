
extern crate sdl2; 

use std::env;
use std::{thread, time};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod chip8;
mod cpu;
mod display;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    println!("The ROM is: {}", filename);

    // initialize screen
    let sdl_context = sdl2::init().unwrap();
    let mut display = display::Display::new(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    // initialize chip8 state
    let mut chip8 = chip8::Chip8::init();
   
    // load rom
    match cpu::load_rom(filename, &mut chip8) {
        Ok(_) => println!("Woah"),
        Err(_) => println!("Nope")
    };

    // 'running: loop {
    //     display.draw_canvas();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // break 'running
                },
                _ => {}
            }
        }
    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    //         // ::std::thread::sleep(Duration::new(5, 1_000_000_000u32));
    // }
    loop {
    display.draw_canvas();
    cpu::execute(cpu::read_opcode(&mut chip8), &mut chip8, &mut display);
    // thread::sleep(time::Duration::from_millis(300));
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }


    // println!("opcode? {:x}", opcode);

}