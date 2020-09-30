use std::env;

mod chip8;
mod cpu;
/* 
Chip8State
|- Memory[4096] <- store rom starting at 512(0x200)
|- Data Register (Vx)
|- Address Register (I)
|- Stack[16]
|- Timers(delay, sound)
|- Font
|- Display
|- Sound
|- program counter[16] <- need to be array????
|- stack pointer
 */



fn main() {

    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    println!("The ROM is: {}", filename);

    let mut chip8 = chip8::Chip8::init();
    match cpu::load_rom(filename, &mut chip8) {
        Ok(_) => println!("Woah"),
        Err(_) => println!("Nope")
    };

    let opcode = cpu::read_op(chip8);


    println!("opcode? {:x}", opcode);

}


// fn load_rom(filename: &String) -> io::Result<()> {
//     let mut memory: [u8; 4096] = [0; 4096];
//     let rom: File = File::open(filename).expect("fuck");
//     let mut i = 0x200;
//     for byte in rom.bytes(){
//         memory[i] = byte.unwrap();
//         println!("index: {} | Byte! {:x}", i, memory[i]);
//         i+=1;
//     }
//     println!("memory 512 {:x} {:x}", memory[0x200], memory[512]);
//     Ok(())
// }