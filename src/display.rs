use sdl2;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;

// font set from multigesture.net
pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub const SCREEN_MULTIPLY: usize = 20;
pub const CHIP8_WIDTH: usize = 64;
pub const CHIP8_HEIGHT: usize = 32;
pub const MONITOR_HEIGHT: usize = CHIP8_HEIGHT * SCREEN_MULTIPLY;
pub const MONITOR_WIDTH: usize = CHIP8_WIDTH * SCREEN_MULTIPLY;

pub struct Display {
    canvas: Canvas<Window>,
} 

// Drawing logic referenced from rust-sdl2 and starrhorne
impl Display {
    pub fn new(ctx: &sdl2::Sdl) -> Display {
        let video_subsystem = ctx.video().unwrap();

        let window = video_subsystem
        .window(
            "CHIP-8",
            MONITOR_WIDTH as u32,
            MONITOR_HEIGHT as u32,
        )
        .position_centered()
        .build()
        .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display {
            canvas: canvas,
        }
    }

    pub fn draw(&mut self, gfx: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (y, row) in gfx.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCREEN_MULTIPLY as u32;
                let y = (y as u32) * SCREEN_MULTIPLY as u32;

                if col == 0 {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                }

                self.canvas.fill_rect(Rect::new(x as i32, y as i32, SCREEN_MULTIPLY as u32, SCREEN_MULTIPLY as u32)).expect("Draw failed");
            }
        }

        self.canvas.present();
    }
}
