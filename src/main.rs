pub mod cpu;
pub mod settings;

use std::fs;
use settings::*;

use cpu::CPU;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            }
            _ => {}
        }
    }
}

fn main() {
    // Init SDL2
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let window = video_subsystem
        .window("Snake Game", X_PIXELS * PIXEL_SIZE, Y_PIXELS * PIXEL_SIZE)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl2_context.event_pump().unwrap();
    canvas.set_scale(PIXEL_SIZE as f32, PIXEL_SIZE as f32).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, X_PIXELS, Y_PIXELS).unwrap();

    // Read program from file
    let program = fs::read("./roms/IBM Logo.ch8").expect("Unable to read file");

    // Init emulator
    let mut cpu = CPU::new();
    cpu.load_program(program);

    // Run emulator
    cpu.run_with_callback(move |cpu| {
        handle_user_input(cpu, &mut event_pump);

        if cpu.update_screen {
            texture.update(None, &cpu.vram, 64 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
    });
}
