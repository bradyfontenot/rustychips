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

pub fn execute(opcode: Opcode){
  match opcode.p {
    0x0000 => ,       // pop stack and set pc to popped address.

    0x1000 => chip8.set_pc(opcode.nnn),       // set 
    0x2000 => chip8.get_memory[opcode.nnn] ,  // push pc to stack, then set pc to nnn
    0x3000 => if chip8.get_v_reg(opcode.x) == opcode.kk {chip8.incrememnt_pc()},
    0x4000 => if chip8.get_v_reg(opcode.x) != opcode.kk {chip8.increment_pc()},
    0x5000 => if chip8.get_v_reg(opcode.x) == chip8.get_v_reg(opcode.y) {chip8.increment_pc()},
    0x6000 => chip8.set_v_reg(opcode.x, opcode.kk),
    0x7000 => chip8.set_v_reg(opcode.x, chip8.get_v_reg() + opcode.kk),    // Vx + NN
    0x8000 => match opcode.n {
      0x0000 => ,
      0x0001 => ,
      0x0002 => ,
      0x0003 => ,
      0x0004 => ,
      0x0005 => ,
      0x0006 => ,
      0x0007 => ,
      0x000E => 
    },
    0x9000 => if chip8.get_v_reg(opcode.x) != chip8.get_v_reg(opcode.y) {chip8.increment_pc()},
    0xA000 => chip8.set_i_reg(opcode.nnn),
    0xB000 => chip.set_pc(opcode.p + opcode.nnn),
    0xC000 => println!("Rand TODO"), // Rand
    0xD000 => println!("Draw TODO"), // Draw
    0xE000 => match opcode.nn {
      0x009E => println!("KeyOp TODO"), // Keyop
      0x00A1 => println!("KeyOp TODO") // Keyop
    },

    0xF000 => match opcode.nn {
      0x0007 => println!("Timer TODO"), // LD Vx, DT
      0x000A => println!("KeyOp TODO"), // LD Vx, K
      0x0015 => println!("Timer TODO"), // LD DT, Vx
      0x0018 => println!("Sound TODO"), // LD ST, Vx
      0x001E => chip8.set_i_reg(chip8.get_i_reg() + opcode.x),  // ADD I, Vx
      0x0029 => print!("Sprite TODO"), // LD F, Vx
      0x0033 => println!("BCD TODO"), // LD B, Vx
      0x0055 => {
        for i in 0..opcode.x {
          chip8.set_memory(chip8.get_i_reg() + i, chip8.get_v_reg(i));
        }
      },
      0x0065 => {
        for i in 0..opcode.x {
          chip8.set_v_reg(i, chip8.get_memory(chip.get_i_reg() + i));
        }
      },
    },


  }
}


/* TODO:
set up timers, keyboard map and look at graphics
write tests for chip8 and cpu.
  */