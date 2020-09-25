pub const V_REGISTER_SIZE: usize = 16;
pub const I_REGISTER_SIZE: usize = 16;
pub const STACK_SIZE: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const FONT_SIZE: usize = 80;

#[derive(Debug, Default)]
struct Chip8{
   v_register: [u8; V_REGISTER_SIZE],      // Vx Register
   i_register: [u16; I_REGISTER_SIZE],     // I Register
   pc: u16,                                // Program Counter
   sp: u8,                                 // Stack Pointer
   stack: [u16; STACK_SIZE],               // Stack
   memory: [u8; MEMORY_SIZE],
   font: [u16; FONT_SIZE]

}