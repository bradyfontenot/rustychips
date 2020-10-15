extern crate sdl2; 

use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use std::time::Duration;

const SCALE_FACTOR: u32 = 20;
const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;



pub struct Display{
    pub canvas: Canvas<Window>,
    pub pixels: [[u8; WIDTH as usize]; HEIGHT as usize]
}


impl Display {

    pub fn new( sdl_context: &sdl2::Sdl) -> Display {
        // let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem.window("Rusty Chips", WIDTH * SCALE_FACTOR, HEIGHT * SCALE_FACTOR)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
        
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display{canvas, pixels: [[0; WIDTH as usize]; HEIGHT as usize]}
    }

    pub fn draw_canvas(&mut self){
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        for y in 0..HEIGHT-1{
            for x in 0..WIDTH-1{
                if self.pixels[y as usize][x as usize] == 0{
                    self.canvas.set_draw_color(pixels::Color::RGB(0,0,0));

                } else{
                    self.canvas.set_draw_color(pixels::Color::RGB(255,255,255));
                }
                let _ = self.canvas.fill_rect(Rect::new((x*SCALE_FACTOR) as i32, (y*SCALE_FACTOR) as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }

        self.canvas.present();
    }
}