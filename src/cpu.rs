use std::fs::File;
use std::io;
use std::io::prelude::*;  
use crate::chip8::Chip8;


// load_rom() loads file into memory
pub fn load_rom(filename: &String, chip8: &mut Chip8) -> io::Result<()> {
  let rom: File = File::open(filename).expect("fuck");
  let mut i = 0x200;
  
  for byte in rom.bytes(){
      chip8.memory[i] = byte.unwrap();
      println!("index: {} | Byte! {:x}", i, chip8.memory[i]);
      i+=1;
  }
  
  println!("memory 512 {:x} {:x}", chip8.memory[0x200], chip8.memory[512]);
  println!("Font Set: {:x}", chip8.memory[0]);
  Ok(())
}

// read_op() gets next opcode in memory and increment program counter
pub fn read_op(mut chip8 :Chip8) -> u8 {
  let opcode = chip8.get_memory(chip8.pc);

  chip8.increment_pc();

  opcode
}

pub fn decode_op(opcode: u8){
  // put match here or pass off to function in another file?
}


/* TODO:
  grab code from memory at pc counter??
  build match statement here...
  send matching op to execute(op)
  */

// pub fn execute(){}