/*
Use 'clap' for command line parsing
    be able to specigy quirks and the desired program, also a debug output file
add debug config mode that allows you to output call stack to file
add sound handler
add clock rate limiter
abstract functions into drivers
add colour options
add better error handling. Don't Panic, return clean errors to the user

Tests
https://github.com/Timendus/chip8-test-suite?tab=readme-ov-file#available-tests
*/

mod config;
mod constants;
mod cpu;
mod drivers;

use config::{Config, ConfigFlags};
use constants::*;
use std::fs;
use drivers::audio_driver::AudioDriver;

use cpu::CPU;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // Test Program to be used (1-8, 0=none)
    #[arg(short, long, default_value_t = 0)]
    test: u8,
}

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
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

// Decrements cpu timers
fn handle_timers(cpu: &mut CPU) {
    if cpu.delay_timer > 0 {
        cpu.delay_timer -= 1;
    }

    if cpu.sound_timer > 0 {
        cpu.sound_timer -= 1;
    }
}

fn handle_sound(cpu: &mut CPU, audio: &AudioDriver) {
    if cpu.sound_timer > 0 {
        audio.start_beep()
    } else {
        audio.stop_beep()
    }
}

fn get_rom(args: Args) -> Vec<u8> {
    let path = if args.test > 0 {
        match args.test {
            1 => "roms/tests/1-chip8-logo.ch8",
            2 => "roms/tests/2-ibm-logo.ch8",
            3 => "roms/tests/3-corax+.ch8",
            4 => "roms/tests/4-flags.ch8",
            5 => "roms/tests/5-quirks.ch8",
            6 => "roms/tests/6-keypad.ch8",
            7 => "roms/tests/7-beep.ch8",
            8 => "roms/tests/8-scrolling.ch8",
            _ => panic!("The test program must be from 1-8")
        }
    } else {
        "roms/IBM Logo.ch8"
    };

    fs::read(path).expect("Unable to read file")
}

fn main() {
    let args = Args::parse();

    // Init SDL2
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let window = video_subsystem
        .window("Chip8 Emulator", X_PIXELS * PIXEL_SIZE, Y_PIXELS * PIXEL_SIZE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas
        .set_scale(PIXEL_SIZE as f32, PIXEL_SIZE as f32)
        .unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, X_PIXELS, Y_PIXELS)
        .unwrap();

    let mut event_pump = sdl2_context.event_pump().unwrap();
    let audio = AudioDriver::new(&sdl2_context);


    // Read program from file
    let config = Config::from(ConfigFlags::DontIndexOverflow | ConfigFlags::JumpWithOffset | ConfigFlags::Shift | ConfigFlags::StoreLoadMem);

    // Init emulator
    let mut cpu = CPU::new(config);

    let rom = get_rom(args);
    cpu.load_program(rom);

    let mut frame_start = std::time::Instant::now();
    let mut timer_count = std::time::Duration::from_secs(0);

    // Run emulator
    cpu.run_with_callback(move |cpu| {
        // Handle program timing
        let frame_end = std::time::Instant::now();

        let frame_time = frame_end - frame_start;

        timer_count += frame_time;

        frame_start = std::time::Instant::now();

        // send input to cpu
        handle_user_input(cpu, &mut event_pump);

        // Update cpu timers @ 60hz
        if timer_count >= std::time::Duration::from_micros(16666) {
            handle_timers(cpu);
            timer_count = std::time::Duration::from_secs(0);
        }

        handle_sound(cpu, &audio);

        // Only updates screen if draw method is called
        if cpu.update_screen {
            texture.update(None, &cpu.vram, 64 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
    });
}
