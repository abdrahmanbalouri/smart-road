/*
   ███████╗ ██████╗ ███╗   ██╗███████╗ ██████╗  ██╗
   ╚══███╔╝██╔═══██╗████╗  ██║██╔════╝██╔═████╗███║
     ███╔╝ ██║   ██║██╔██╗ ██║█████╗  ██║██╔██║╚██║
    ███╔╝  ██║   ██║██║╚██╗██║██╔══╝  ████╔╝██║ ██║
   ███████╗╚██████╔╝██║ ╚████║███████╗╚██████╔╝ ██║
   ╚══════╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝ ╚═════╝  ╚═╝

   Author : abalouri
   File   : main.rs
   Project: smart-road
   Date   : 3/02/2026
*/

use rand::Rng;
 use sdl2::event::Event;
// #[cfg(feature = "sdl_image")]
// use sdl2::image::{self, LoadTexture};
// #[cfg(feature = "sdl_ttf")]
// use sdl2::ttf;
 use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::rect::Rect;
// use std::collections::VecDeque;
use std::time::Duration;
// #[cfg(feature = "sdl_ttf")]
// use std::path::Path;

mod help;
//use help::*;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("smart-road", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
