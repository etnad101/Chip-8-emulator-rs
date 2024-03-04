pub mod config;
pub mod constants;
pub mod cpu;

use config::Config;
use constants::*;
use std::fs;

use cpu::CPU;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    println!("Inputs: {:#016b}", cpu.input);
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),

            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::X) => cpu.input |= 1 << 0x0,
                Some(Keycode::Num1) => cpu.input |= 1 << 0x1,
                Some(Keycode::Num2) => cpu.input |= 1 << 0x2,
                Some(Keycode::Num3) => cpu.input |= 1 << 0x3,
                Some(Keycode::Q) => cpu.input |= 1 << 0x4,
                Some(Keycode::W) => cpu.input |= 1 << 0x5,
                Some(Keycode::E) => cpu.input |= 1 << 0x6,
                Some(Keycode::A) => cpu.input |= 1 << 0x7,
                Some(Keycode::S) => cpu.input |= 1 << 0x8,
                Some(Keycode::D) => cpu.input |= 1 << 0x9,
                Some(Keycode::Z) => cpu.input |= 1 << 0xA,
                Some(Keycode::C) => cpu.input |= 1 << 0xB,
                Some(Keycode::Num4) => cpu.input |= 1 << 0xC,
                Some(Keycode::R) => cpu.input |= 1 << 0xD,
                Some(Keycode::F) => cpu.input |= 1 << 0xE,
                Some(Keycode::V) => cpu.input |= 1 << 0xF,
                _ => (),
            },

            Event::KeyUp { keycode, .. } => match keycode {
                Some(Keycode::X) => cpu.input &= 0b1111_1111_1111_1110,
                Some(Keycode::Num1) => cpu.input &= 0b1111_1111_1111_1101,
                Some(Keycode::Num2) => cpu.input &= 0b1111_1111_1111_1011,
                Some(Keycode::Num3) => cpu.input &= 0b1111_1111_1111_0111,
                Some(Keycode::Q) => cpu.input &= 0b1111_1111_1110_1111,
                Some(Keycode::W) => cpu.input &= 0b1111_1111_1101_1111,
                Some(Keycode::E) => cpu.input &= 0b1111_1111_1011_1111,
                Some(Keycode::A) => cpu.input &= 0b1111_1111_0111_1111,
                Some(Keycode::S) => cpu.input &= 0b1111_1110_1111_1111,
                Some(Keycode::D) => cpu.input &= 0b1111_1101_1111_1111,
                Some(Keycode::Z) => cpu.input &= 0b1111_1011_1111_1111,
                Some(Keycode::C) => cpu.input &= 0b1111_0111_1111_1111,
                Some(Keycode::Num4) => cpu.input &= 0b1110_1111_1111_1111,
                Some(Keycode::R) => cpu.input &= 0b1101_1111_1111_1111,
                Some(Keycode::F) => cpu.input &= 0b1011_1111_1111_1111,
                Some(Keycode::V) => cpu.input &= 0b0111_1111_1111_1111,
                _ => (),
            },

            _ => (),
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
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl2_context.event_pump().unwrap();
    canvas
        .set_scale(PIXEL_SIZE as f32, PIXEL_SIZE as f32)
        .unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, X_PIXELS, Y_PIXELS)
        .unwrap();

    // Read program from file
    let program = fs::read("./roms/IBM Logo.ch8").expect("Unable to read file");

    let config = Config::default();

    // Init emulator
    let mut cpu = CPU::new(config);
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
