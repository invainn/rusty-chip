use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keypad {
    events: sdl2::EventPump,
}

impl Keypad {
    pub fn new(ctx: &sdl2::Sdl)-> Keypad {
        Keypad {
            events: ctx.event_pump().unwrap()
        }
    }

    pub fn poll(&mut self) -> Result<[bool; 16], ()> {
        for event in self.events.poll_iter() {
            if let Event::Quit { .. } = event {
                return Err(());
            };
        }

        let keys: Vec<Keycode> = self.events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xC),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xD),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xE),
                Keycode::Z => Some(0xA),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xB),
                Keycode::V => Some(0xF),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys[i] = true;
            }
        }

        Ok(chip8_keys)
    }
}
