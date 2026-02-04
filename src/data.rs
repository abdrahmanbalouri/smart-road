use sdl2::messagebox::{show_simple_message_box, MessageBoxFlag};
use std::time::Duration;

pub fn write_stats(
    nb_cars: i32, 
    collision_just: i32, 
    collision: i32, 
    max_speed: i32, 
    min_speed: i32, 
    max_timer: &Duration, 
    min_timer: &Duration
) {
    let max_t = format!("{:.2}s", max_timer.as_secs_f32());
    let min_t = format!("{:.2}s", min_timer.as_secs_f32());

    let text = format!(
        "📊 TRAFFIC SIMULATION REPORT\n\
         ------------------------------------------\n\
         🚗 Total Vehicles:      {}\n\
         ⚠️ Close Calls:         {}\n\
         💥 Total Collisions:    {}\n\
         ------------------------------------------\n\
         🚀 Max Velocity:        {} px/s\n\
         🐢 Min Velocity:        {} px/s\n\
         ------------------------------------------\n\
         🕒 Slowest Vehicle:     {}\n\
         ⚡ Fastest Vehicle:     {}\n\
         ------------------------------------------",
        nb_cars, collision_just, collision, max_speed, min_speed, max_t, min_t
    );

    _ = show_simple_message_box(
        MessageBoxFlag::INFORMATION,
        "🚦 Smart Road - Final Stats",
        &text,
        None,
    );
}