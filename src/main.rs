mod cpu;
mod input;
mod audio;
mod display;

use std::thread;
use std::time::Duration;

use cpu::CPU;
use audio::Audio;
use display::Display;
use input::Keypad;

use std::env;

fn main() {
    let mut cpu = CPU::new();
    let sdl_context = sdl2::init().unwrap();

    let mut disp = Display::new(&sdl_context);
    let sound = Audio::new(&sdl_context);
    let mut keypad = Keypad::new(&sdl_context);

    let args: Vec<String> = env::args().collect();

    // Initialize the CPU and load the game into memory
    cpu.initialize(args[1].to_string());

    while let Ok(kp) = keypad.poll() {

        for _ in 1..=9 {
            cpu.emulate_cycle(kp);
        }

        // Wait for some time
        thread::sleep(Duration::from_millis(16));
       
        // Handle drawing if there is a need
        if cpu.draw_flag {
            disp.draw(&cpu.gfx);
            cpu.draw_flag = false;
        }

        if cpu.sound_timer > 0 {
            sound.play();
        } else {
            sound.stop();
        }

        // Decrement the timer after 60hz is up
        // this happens after 60hz since there are 9 cycles per timer decrement
        cpu.decrement_timers();
    }
}
