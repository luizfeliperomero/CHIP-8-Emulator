use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowContext};
use sdl2::Sdl;
use sdl2::render::TextureCreator;
use std::time::Instant;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const FPS: u64 = 60;

pub trait DisplayTrait {
    fn draw(&mut self) -> bool;
    fn clear(&mut self);
    fn get_pixels(&self) -> [u8; WIDTH * HEIGHT * 3];
    fn set_pixels(&mut self, value: [u8; WIDTH * HEIGHT * 3]);
    fn get_pixel_byte(&self, index: usize) -> u8;
    fn set_pixel_byte(&mut self, index: usize, value: u8);
}

pub struct Display {
    pub title: String,
    pixels: [u8; WIDTH * HEIGHT * 3],
    canvas: Canvas<Window>,
    pub last_updated: Instant,
    texture_creator: TextureCreator<WindowContext>
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Self {
        let title = "CHIP-8 Emulator by luizf".to_string();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title.as_str(), WIDTH as u32 * 10, HEIGHT as u32 * 10)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        Self {
            title,
            pixels: [0; WIDTH * HEIGHT * 3],
            canvas,
            last_updated: Instant::now(),
            texture_creator
        }
    }
}

impl DisplayTrait for Display {
    fn draw(&mut self) -> bool {
        let now = Instant::now();
        let mut texture = self.texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, WIDTH as u32, HEIGHT as u32)
            .expect("Couldn't create texture");
        let _ = texture.update(None, &self.pixels, WIDTH * 3);
        let _ = self.canvas.copy(&texture, None, None);
        self.canvas.present();
        self.last_updated = now;
        return true;
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
    fn get_pixel_byte(&self, index: usize) -> u8 {
        self.pixels[index]
    }
    fn set_pixel_byte(&mut self, index: usize, value: u8) {
        self.pixels[index] = value;
    }
}
