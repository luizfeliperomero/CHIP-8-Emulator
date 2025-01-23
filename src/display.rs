use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;
use std::time::{Duration, Instant};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const FPS: u64 = 60;

pub trait DisplayTrait {
    fn draw(&mut self) -> bool;
    fn clear(&mut self);
    fn get_pixels(&self) -> [u8; WIDTH * HEIGHT * 3];
    fn set_pixels(&mut self, value: [u8; WIDTH * HEIGHT * 3]);
    fn get_pixel(&self, index: usize) -> u8;
    fn set_pixel(&mut self, index: usize, value: u8);
}

pub struct Display {
    pub title: String,
    pixels: [u8; WIDTH * HEIGHT * 3],
    canvas: Canvas<Window>,
    pub last_updated: Instant,
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {
        let title = "CHIP-8 Emulator".to_string();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title.as_str(), WIDTH as u32 * 10, HEIGHT as u32 * 10)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Self {
            title,
            pixels: [0; WIDTH * HEIGHT * 3],
            canvas,
            last_updated: Instant::now(),
        }
    }
}

impl DisplayTrait for Display {
    fn draw(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_updated) >= Duration::from_millis(1000 / FPS) {
            let texture_creator = self.canvas.texture_creator();
            let mut texture = texture_creator
                .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
                .expect("Couldn't create texture");
            let _ = texture.update(None, &self.pixels, WIDTH * 3);
            let _ = self.canvas.copy(&texture, None, None);
            self.canvas.present();
            self.last_updated = Instant::now();
            return true;
        }
        false
    }
    fn clear(&mut self) {
        self.pixels = [0; WIDTH * HEIGHT * 3];
    }
    fn get_pixels(&self) -> [u8; WIDTH * HEIGHT * 3] {
        self.pixels
    }
    fn set_pixels(&mut self, value: [u8; WIDTH * HEIGHT * 3]) {
        self.pixels = value;
    }
    fn get_pixel(&self, index: usize) -> u8 {
        self.pixels[index]
    }
    fn set_pixel(&mut self, index: usize, value: u8) {
        self.pixels[index] = value;
    }
}
