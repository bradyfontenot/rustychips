extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

pub struct Input{
    pub event: sdl2::EventPump,
    pub key_press: u8
}

impl Input {
    pub fn init(sdl_context: &sdl2::Sdl) -> Input{
        Input{
            event: sdl_context.event_pump().unwrap(),
            key_press: 0
        }
    }

    pub fn handle_key_event(&mut self) -> u8 {
        for event in self.event.poll_iter(){
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return 0
                },
                Event::KeyDown{keycode: Some(key), ..} => {
                    self.key_press = match key {
                        Keycode::Num1 => 0x1,
                        Keycode::Num2 => 0x2,
                        Keycode::Num3 => 0x3,
                        Keycode::Num4 => 0xc,
                        Keycode::Q => 0x4,
                        Keycode::W => 0x5,
                        Keycode::E => 0x6,
                        Keycode::R => 0xd,
                        Keycode::A => 0x7,
                        Keycode::S => 0x8,
                        Keycode::D => 0x9,
                        Keycode::F => 0xe,
                        Keycode::Z => 0xa,
                        Keycode::X => 0x0,
                        Keycode::C => 0xb,
                        Keycode::V => 0xf,
                        Keycode::Up => 0x2,
                        Keycode::Down => 0x8,
                        Keycode::Left => 0x4,
                        Keycode::Right => 0x6,
                        _ => 0
                    }
                },
                _ => continue 
            }
            // self.key_press = 0;
        }
        1
    }
}