use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::EventPump;
use std::time::Duration;

const SCALE_FACTOR: u32 = 30;
pub const WIDTH: u32 = 64 * SCALE_FACTOR;
pub const HEIGHT: u32 = 32 * SCALE_FACTOR;

pub struct Display {
    pub title: String,
    pub pixels: [[bool; WIDTH as usize]; HEIGHT as usize],
    canvas: Canvas<Window>,
    pub event_pump: EventPump
}

impl Default for Display {
    fn default() -> Self {
        let title = "CHIP-8 Emulator".to_string();
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(
            title.as_str(),
            WIDTH,
            HEIGHT
        )
        .position_centered()
        .build()
        .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Self {
            title,
            pixels: [[false; WIDTH as usize]; HEIGHT as usize],
            canvas,
            event_pump
        }
    }
}

impl Display {
    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear(); 
        self.canvas.set_draw_color(Color::WHITE);
        for y in 0..HEIGHT{
            for x in 0..WIDTH{
                if self.pixels[y as usize][x as usize] {
                    let rect = Rect::new(
                        (x as u32 * SCALE_FACTOR) as i32,
                        (y as u32 * SCALE_FACTOR) as i32,
                        SCALE_FACTOR,
                        SCALE_FACTOR,
                    );
                    self.canvas.fill_rect(rect).unwrap();
                }
            }
        }
        self.canvas.present();
        ::std::thread::sleep(Duration::from_millis(16));
    }
}

