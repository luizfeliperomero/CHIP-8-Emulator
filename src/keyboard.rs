use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};

pub const KEYS: [Keycode; 16] = [
    Keycode::X,
    Keycode::Num1,
    Keycode::Num2,
    Keycode::Num3,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::Z,
    Keycode::C,
    Keycode::Num4,
    Keycode::R,
    Keycode::F,
    Keycode::V,
];
pub struct Keyboard {
    keys_state: u16,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys_state: 0 }
    }
}

impl Keyboard {
    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys_state & (1 << key) == 1 << key
    }
    pub fn press(&mut self, key: u8) {
        self.keys_state |= 1 << key;
    }
    pub fn release(&mut self, key: u8) {
        self.keys_state &= !(1 << key);
    }
    pub fn update(&mut self, sdl_context: &Sdl) {
        let mut event_pump = sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        if let Some(key_num) = map_key_to_u8(key) {
                            self.press(key_num);
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        if let Some(key_num) = map_key_to_u8(key) {
                            self.release(key_num);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    pub fn is_any_pressed(&self) -> bool {
        self.keys_state != 0
    }
}

pub fn map_key_to_u8(key: Keycode) -> Option<u8> {
    KEYS.iter()
        .enumerate()
        .find_map(|(i, k)| if *k == key { Some(i) } else { None })
        .map(|i| i as u8)
}
