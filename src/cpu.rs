use std::fs::File;
use std::io;
use std::io::prelude::*;  
use crate::chip8::Chip8;
use crate::input::Input;
use rand::Rng;
// use rand::prelude::*;

use crate::display::Display;

pub struct Opcode{
  code: u16,    // full opcode
  p: u16,       // highest bit
  x: u8,        // low bit of the high byte
  y: u8,        // high bit of the low byte
  kk: u8,       // lowest byte
  nnn: u16,     // lowest 12 bits
  n: u8        // lowest bit
}

// load_rom() loads file into memory
pub fn load_rom(filename: &String,  chip8: &mut Chip8) -> io::Result<()> {
  let rom: File = File::open(filename).expect("fuck");
  let mut i: u16 = 0x200;
  
  for byte in rom.bytes(){
      chip8.set_memory(i, byte.unwrap());
      
      // Debug printout
      // println!("index: {} {:x} | Byte! {:x}", i, i, chip8.memory(i));
      
      i+=1;
  }

  // Printing for debug purposes
  // println!("memory 512 {:x} {:x}", chip8.memory(0x200), chip8.memory(512));
  // println!("Font Set: {:x}", chip8.memory(0));
  Ok(())
}

// read_opcode() gets next opcode in memory and increments program counter
pub fn read_opcode(chip8: &mut Chip8) -> Opcode {
  let opcode = build_opcode(chip8);
  
  // increment pc by 2
  chip8.pc_plus_2();

  // println!("Opcode: {:x}", opcode.code);
  // println!("Opcode.p: {:x}", opcode.p);
  // println!("Opcode.x: {:x}", opcode.x);
  // println!("Opcode.y: {:x}", opcode.y);
  // println!("Opcode.kk: {:x}", opcode.kk);
  // println!("Opcode.nnn: {:x}", opcode.nnn);
  // println!("Opcode.n: {:x}", opcode.n);
  // println!("**************************");

  opcode
}

// build_opcode() - opcodes are 2 bytes but bytes are read from memory one at a time
// so byte pairs in sequence need to be combined to build a full opcode
fn build_opcode(chip8: &mut Chip8) -> Opcode {
  let hi_byte: u8 = chip8.memory(chip8.pc());
  let low_byte: u8 = chip8.memory(chip8.pc() + 1);
  let opcode: u16 = (hi_byte as u16 * 0x100) + (low_byte as u16);

  Opcode{
    code: opcode,
    p:    opcode & 0xF000,
    x:    ((opcode & 0x0F00) >> 8) as u8,
    y:    ((opcode & 0x00F0) >> 4) as u8,
    kk:   (opcode & 0xFF) as u8,
    nnn:  opcode & 0xFFF,
    n:    (opcode & 0xF) as u8
  }
}

pub fn execute(opcode: Opcode, chip8: &mut Chip8, display: &mut Display, input: &mut Input){
  match opcode.p {
    0x0000 => match opcode.kk {
      0x00E0 => display.canvas.clear(),
      0x00EE => {
        let addr = chip8.pop_stack();
        chip8.set_pc(addr)
      },
      _ => println!("0x000 Not Handled")
    },
    0x1000 => chip8.set_pc(opcode.nnn),
    0x2000 => {
      chip8.push_stack(chip8.pc());
      chip8.set_pc(opcode.nnn);
    },
    0x3000 => if chip8.v_register(opcode.x) == opcode.kk {chip8.pc_plus_2()},
    0x4000 => if chip8.v_register(opcode.x) != opcode.kk {chip8.pc_plus_2()},
    0x5000 => if chip8.v_register(opcode.x) == chip8.v_register(opcode.y) {chip8.pc_plus_2()},
    0x6000 => chip8.set_v_reg(opcode.x, opcode.kk),
    0x7000 => match(chip8.v_register(opcode.x)).overflowing_add(opcode.kk){
      (val, true) => chip8.set_v_reg(opcode.x, val),  // Vx + NN
      (val, false) => chip8.set_v_reg(opcode.x, val)  // Vx + NN
    },
    0x8000 => match opcode.n {
      0x0000 => chip8.set_v_reg(opcode.x, chip8.v_register(opcode.y) as u8),
      0x0001 => chip8.set_v_reg(opcode.x, (chip8.v_register(opcode.x) | chip8.v_register(opcode.y)) as u8),
      0x0002 => chip8.set_v_reg(opcode.x, (chip8.v_register(opcode.x) & chip8.v_register(opcode.y)) as u8),
      0x0003 => chip8.set_v_reg(opcode.x, (chip8.v_register(opcode.x) ^ chip8.v_register(opcode.y)) as u8),
      0x0004 => match (chip8.v_register(opcode.x)).overflowing_add(chip8.v_register(opcode.y)){
        (val, true) => {chip8.set_v_reg(opcode.x, val); 
          chip8.set_v_reg(0xF, 1);
        },
        (val, false) => {chip8.set_v_reg(opcode.x, val); 
          chip8.set_v_reg(0xF, 0);
        }
      },
      0x0005 => match (chip8.v_register(opcode.x)).overflowing_sub(chip8.v_register(opcode.y)){
        (val, true) => {chip8.set_v_reg(opcode.x, val); 
          chip8.set_v_reg(0xF, 0);
        },
        (val, false) => {chip8.set_v_reg(opcode.x, val); 
          chip8.set_v_reg(0xF, 1);
        },
      },
      0x0006 => {
        chip8.set_v_reg(0xF, chip8.v_register(opcode.x) & 1);
        chip8.set_v_reg(opcode.x, chip8.v_register(opcode.x) >> 1);
      }, 
      0x0007 => match (chip8.v_register(opcode.y)).overflowing_sub(chip8.v_register(opcode.x)){
        (val, true) => {chip8.set_v_reg(opcode.x, val); 
          chip8.set_v_reg(0xF, 0);
        },
        (val, false) => {chip8.set_v_reg(opcode.x, val); 
          chip8.set_v_reg(0xF, 1);
        }
      },
      0x000E => {
        let vf = (chip8.v_register(opcode.x) >> 7) & 1;
        chip8.set_v_reg(0xF, vf);
        chip8.set_v_reg(opcode.x, chip8.v_register(opcode.x) << 1);
      },
      _ => println!("0x8000 Not Handled")
    },
    0x9000 => if chip8.v_register(opcode.x) != chip8.v_register(opcode.y) {chip8.pc_plus_2()},
    0xA000 => chip8.set_i_reg(opcode.nnn),
    0xB000 => chip8.set_pc(opcode.nnn + chip8.v_register(0) as u16),
    0xC000 => {
      let mut rng = rand::thread_rng();
      chip8.set_v_reg(opcode.x, rng.gen::<u8>() & opcode.kk)
    },
    0xD000 => {
      let addr = chip8.i_register();
      let mut y_coord: usize ;
      let mut x_coord: usize;
      let mut sprite_byte: u8;
      // let mut sprite_pixel: u8;
      let mut vf = 0;
      chip8.set_v_reg(0xF, 0);
      
      for y in 0..opcode.n {
        y_coord = ((chip8.v_register(opcode.y) + y) % 32) as usize;
        sprite_byte = chip8.memory(addr + y as u16);
      
        for x in 0..8{
          x_coord = ((chip8.v_register(opcode.x) as usize + x as usize) % 64) as usize;
          let old_pixel = display.pixels[y_coord][x_coord];
          let new_pixel = (sprite_byte >> 7-x) & 1;
          display.pixels[y_coord][x_coord] = old_pixel ^ new_pixel;
          vf |= new_pixel & old_pixel;
          chip8.set_v_reg(0xF, vf);
          println!("VF: {}", chip8.v_register(0xF));
        }

      }
    },
    0xE000 => match opcode.kk {
      // if keyOP down event = Vx skip instr (i.e. pc += 2)
      0x009E => {
        if input.key_press == chip8.v_register(opcode.x) {chip8.pc_plus_2()};
        println!("9E => Want: {}, Pressed: {}", chip8.v_register(opcode.x), input.key_press);
      },
      0x00A1 => {
        if input.key_press != chip8.v_register(opcode.x) {chip8.pc_plus_2()};
        println!("A1 => Want: {}, Pressed: {}", chip8.v_register(opcode.x), input.key_press);
      },
      _ => ()
    },
    0xF000 => match opcode.kk {
      0x0007 => chip8.set_v_reg(opcode.x, chip8.delay_timer() as u8),
      0x000A => {
        println!("Looping Key: {}", input.key_press);
        input.key_press = 0;
        while input.key_press == 0{
          input.handle_key_event(); 
        };
        chip8.set_v_reg(opcode.x, input.key_press);

      },
      0x0015 => chip8.set_delay_timer(chip8.v_register(opcode.x)), 
      0x0018 => chip8.set_sound_timer(chip8.v_register(opcode.x)), 
      0x001E => chip8.set_i_reg(chip8.i_register() + chip8.v_register(opcode.x) as u16),
      // TODO: FIX \/ *****FONT*****
      0x0029 => chip8.set_i_reg(chip8.fonts[chip8.v_register(opcode.x) as usize] as u16),
      0x0033 => {
        let register = chip8.v_register(opcode.x);

        // hundreds - / 100
        let hundreds = register / 100;
        chip8.set_memory(chip8.i_register(), hundreds);

        // tens - / 10, % 10
        let tens = (register / 10 ) % 10;
        chip8.set_memory(chip8.i_register() + 1, tens);

        // ones - % 10
        let ones = register % 10;
        chip8.set_memory(chip8.i_register() + 2, ones);
      },
      0x0055 => {
        for i in 0..opcode.x + 1 {
          chip8.set_memory(chip8.i_register() + i as u16, chip8.v_register(i));
        }
      },
      0x0065 => {
        for i in 0..opcode.x + 1 {
          chip8.set_v_reg(i, chip8.memory(chip8.i_register() + i as u16));
        }
      },
      _ => ()
    },
    _ => ()
  }
}