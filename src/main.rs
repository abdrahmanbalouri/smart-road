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
use sdl2::pixels::PixelFormatEnum;
//  use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::pixels::PixelFormatEnum;
// use sdl2::rect::Rect;
// use std::collections::VecDeque;
use std::time::Duration;
// #[cfg(feature = "sdl_ttf")]
// use std::path::Path;

mod help;
//use help::*;
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("smart-road", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Load image with the `image` crate and upload into a streaming texture
    let img = image::open("src/img/car.png").unwrap().into_rgba8();
    let (w, h) = img.dimensions();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, w, h)
        .unwrap();

    let pixels = img.into_raw();

    texture
        .with_lock(None, |buf, pitch| {
            let row = (w as usize) * 4;
            for y in 0..h as usize {
                let src = y * row;
                let dst = y * pitch;
                buf[dst..dst + row].copy_from_slice(&pixels[src..src + row]);
            }
        })
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        // draw car texture in top-left (example)
          canvas.copy(&texture, None, None).ok();
        // Drawing code would go here
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_00000u32 / 60));
    }

    Ok(())
}
