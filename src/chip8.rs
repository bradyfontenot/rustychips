pub const V_REGISTER_SIZE: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const FONT_SIZE: usize = 80;

pub const FONT_SET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub struct Chip8{
  v_register: [u8; V_REGISTER_SIZE],      // V Register
  i_register: u16,                        // I Register
  pc: u16,                                // Program Counter
  sp: u8,                                 // Stack Pointer
  stack: [u16; STACK_SIZE],               // Stack
  memory: [u8; MEMORY_SIZE],              // RAM
  sound_timer: u8,
  delay_timer: u8,
  pub fonts: [u8; 80]
}


impl Chip8 {

  // constructor
  pub fn init() -> Chip8 {
    let mut chippy = Chip8{
      v_register: [0; V_REGISTER_SIZE],
      i_register: 0x000,
      pc: 0x200,
      sp: 0x000,
      stack: [0; 16],
      memory: [0; 4096],
      sound_timer: 0,
      delay_timer: 0,
      fonts: FONT_SET
    };
  
    Chip8::load_font_set(&mut chippy);
  
    chippy
  }

  // Loads font set into memory
  // stored in unused portion of memory below 0x200
  fn load_font_set(chippy: &mut Chip8){
    for i in 0..FONT_SIZE{
      chippy.memory[i] = FONT_SET[i];
    }  
  }

  // store byte in memory addr
  pub fn set_memory(&mut self, addr: u16, val: u8){
    self.memory[addr as usize] = val;
  }

  // get byte from memory addr
  pub fn get_memory(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  // store byte in V Register
  pub fn set_v_reg(&mut self, addr: u16, val: u8){
    self.v_register[addr as usize] = val;
  }

  // get byte from V Register
  pub fn get_v_reg(&self, addr: u16) -> u8 {
    self.v_register[addr as usize]
  }

  // store byte in I Register
  pub fn set_i_reg(&mut self, addr: u16){
    self.i_register = addr;
  }

  // get byte from I Register
  pub fn get_i_reg(&self) -> u16 {
    self.i_register
  }

  // store addr in PC
  // pub fn set_pc(){}

  // get addr from PC
  pub fn get_pc(&self) -> u16 {
    self.pc
  }

  // increment program counter by 2
  pub fn increment_pc(&mut self){
    self.pc = self.pc + 2;
  }
  // store addr in SP
  // pub fn set_sp(){}

  // get addr from sp
  // pub fn get_sp(){}

  // push instruction to stack
  // pub fn push_stack(){}

  // get instruction from stack
  // pub fn pop_stack(){}


}






/* 
set v_register
get v_register
set i_register
get i_register
 */

//  ******TESTS******

/*
#[cfg(test)]
mod tests{
  #[test]
  fn test_(){
    // test here
  }
}
*/