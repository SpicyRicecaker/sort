extern crate sdl2;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sort::heap::Heap;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut rng = rand::thread_rng();
    let mut data = Vec::with_capacity(100);
    for _ in 0..100 {
        data.push(rng.gen_range(1..=100));
    }

    // let mut data = vec![5, 2, 3, 4, 5];
    let len = data.len() as u32;

    let mut heap = Heap::new(data);

    let size_pillar_x = 1280 / len;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        heap.tick();
        heap.render(&mut canvas, size_pillar_x);
        // canvas.set_draw_color(Color::RGB(255, 255, 255));
        // canvas.fill_rect(Rect::new(0, 0, 50, 720)).unwrap();
        // canvas.fill_rect(Rect::new(0, 0, 50, 720)).unwrap();

        canvas.present();
        // 1 fps, since with 60fps the sorting algorithm will run too fast. If
        // we want to implement another internal timer for the sorting algorithm
        // itself at 60 fps, that could work, too
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
