/*
   ███████╗ ██████╗ ███╗   ██╗███████╗ ██████╗  ██╗
   ╚══███╔╝██╔═══██╗████╗  ██║██╔════╝██╔═████╗███║
     ███╔╝ ██║   ██║██╔██╗ ██║█████╗  ██║██╔██║╚██║
    ███╔╝  ██║   ██║██║╚██╗██║██╔══╝  ████╔╝██║ ██║
   ███████╗╚██████╔╝██║ ╚████║███████╗╚██████╔╝ ██║
   ╚══════╝ ╚═════╝ ╚═╝  ╚═══╝╚══════╝ ╚═════╝  ╚═╝

   Author : abalouri
   Author : azraji
   File   : main.rs
   Project: smart-road
   Date   : 3/02/2026
*/

use image::{Rgba, RgbaImage};
use rusttype::{Font, Scale};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;


pub fn draw_confirm_exit(
    canvas: &mut Canvas<Window>,
    nbr_cars: i32,
    max_velocity: f32,
    min_velocity: f32,
    max_timer: &Duration,
    min_timer: &Duration,
    close_calls: i32,
) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(Rect::new(200, 220, 400, 300)).unwrap();

    let texts = vec![
        format!("Total cars: {}", nbr_cars),
        format!("Max_Velocity: {} ", max_velocity),
        format!("Min_Velocity: {} " , min_velocity),
        format!("Max time: {:.2?}", max_timer),
        format!("Min time: {:.2?}", min_timer),
        format!("Close calls: {}", close_calls),
    ];

    let mut y = 220;
    for text in texts {
        let img = text_to_image(&text);
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_static(
                Some(sdl2::pixels::PixelFormatEnum::RGBA8888),
                img.width(),
                img.height(),
            )
            .unwrap();
        texture
            .update(None, &img, 4 * img.width() as usize)
            .unwrap();
        canvas
            .copy(
                &texture,
                None,
                Some(Rect::new(220, y, img.width(), img.height())),
            )
            .unwrap();
        y += 50;
    }

    canvas.present();
}

pub fn text_to_image(text: &str) -> RgbaImage {
    let font_data = include_bytes!("../assets/DejaVuSans1.ttf");

    let font = match Font::try_from_bytes(font_data as &[u8]) {
        Some(f) => f,
        None => {
            eprintln!("Failed to load font from bytes!");
            return RgbaImage::new(1, 1);
        }
    };

    let scale = Scale::uniform(20.0);
    let width = 300;
    let height = 40;
    let mut img = RgbaImage::new(width, height);

    for (i, c) in text.chars().enumerate() {
        let v_metrics = font.v_metrics(scale);
        let glyph = font.glyph(c).scaled(scale).positioned(rusttype::point(
            5.0 + i as f32 * 15.0,
            20.0 + v_metrics.ascent,
        ));
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = x + bb.min.x as u32;
                let py = y + bb.min.y as u32;
                if px < width && py < height {
                    img.put_pixel(px, py, Rgba([0, 0, 0, (v * 255.0) as u8]));
                }
            });
        }
    }

    img
}
