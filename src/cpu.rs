use std::fs::File;
use std::io;
use std::io::prelude::*;  
use crate::chip8::Chip8;


pub struct Opcode{
  code: u16,
  p: u16,
  x: u16,
  y: u16,
  kk: u16,
  nnn: u16,
  n: u16
}

// load_rom() loads file into memory
pub fn load_rom(filename: &String,  chip8: &mut Chip8) -> io::Result<()> {
  let rom: File = File::open(filename).expect("fuck");
  let mut i: u16 = 0x200;
  
  for byte in rom.bytes(){
      chip8.set_memory(i, byte.unwrap());
      
      // Debug printout
      println!("index: {} {:x} | Byte! {:x}", i, i, chip8.get_memory(i));
      
      i+=1;
  }

  // Printing for debug purposes
  println!("memory 512 {:x} {:x}", chip8.get_memory(0x200), chip8.get_memory(512));
  println!("Font Set: {:x}", chip8.get_memory(0));
  Ok(())
}

// read_opcode() gets next opcode in memory and increments program counter
pub fn read_opcode(chip8: &mut Chip8) -> Opcode {
  let opcode = build_opcode(chip8);
  
  // increment pc by 2
  chip8.increment_pc();

  println!("Opcode: {:x}", opcode.code);
  println!("Opcode.p: {:x}", opcode.p);
  println!("Opcode.x: {:x}", opcode.x);
  println!("Opcode.y: {:x}", opcode.y);
  println!("Opcode.kk: {:x}", opcode.kk);
  println!("Opcode.nnn: {:x}", opcode.nnn);
  println!("Opcode.n: {:x}", opcode.n);

  opcode
}

// build_opcode() - opcodes are 2 bytes but bytes are read from memory one at a time
// so byte pairs in sequence need to be combined to build a full opcode
fn build_opcode(chip8: &mut Chip8) -> Opcode {
  let hi_byte: u8 = chip8.get_memory(chip8.get_pc());
  let low_byte: u8 = chip8.get_memory(chip8.get_pc() + 1);
  let opcode: u16 = (hi_byte as u16 * 0x100) + (low_byte as u16);

  Opcode{
    code: opcode,
    p:    opcode & 0xF000,
    x:    opcode & 0x0F00,
    y:    opcode & 0x00F0,
    kk:   opcode & 0xFF,
    nnn:  opcode & 0xFFF,
    n:    opcode & 0xF
  }
}

}


/* TODO:
  grab code from memory at pc counter??
  build match statement here...
  send matching op to execute(op)
  */

// pub fn execute(){}