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
use drivers::audio_driver::AudioDriver;
use drivers::input_driver::InputManager;
use drivers::rom_driver::{Program, ProgramType};

use clap::Parser;
use cpu::CPU;
use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;

#[derive(Parser, Debug)]
struct Args {
    // Test Program to be used (1-8, 0=none)
    #[arg(short, long, default_value_t = 0)]
    test: u8,
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

fn main() {
    let args = Args::parse();

    // Init SDL2
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Chip8 Emulator",
            X_PIXELS * PIXEL_SIZE,
            Y_PIXELS * PIXEL_SIZE,
        )
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

    // -----------------------------------------------------------------------------------

    // Init emulator
    let mut cpu = CPU::new(Config::default());

    let program_path: ProgramType;

    if args.test > 0 {
        program_path = ProgramType::Test(args.test);
    } else {
        program_path = ProgramType::Path("./roms/IBM Logo.ch8".to_string());
    }

    let program = Program::new(program_path);

    cpu.load_program(program.bytes);

    let mut frame_start = std::time::Instant::now();
    let mut timer_count = std::time::Duration::from_secs(0);

    let mut input = InputManager::new();

    // -----------------------------------------------------------------------------------

    // Run emulator
    cpu.run_with_callback(move |cpu| {
        // Handle program timing
        let frame_end = std::time::Instant::now();

        let frame_time = frame_end - frame_start;

        timer_count += frame_time;

        frame_start = std::time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                _ => {}
            }
            input.handle_keyboard_input(event);
        }

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
