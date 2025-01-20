use sdl2::keyboard::Keycode;

pub const KEYS: [(Keycode, u8); 16] = [
    (Keycode::Num1, 0x1),
    (Keycode::Num2, 0x2),
    (Keycode::Num3, 0x3),
    (Keycode::Num4, 0xC),
    (Keycode::Q, 0x4),
    (Keycode::W, 0x5),
    (Keycode::E, 0x6),
    (Keycode::R, 0xD),
    (Keycode::A, 0x7),
    (Keycode::S, 0x8),
    (Keycode::D, 0x9),
    (Keycode::F, 0xE),
    (Keycode::Z, 0xA),
    (Keycode::X, 0x0),
    (Keycode::C, 0xB),
    (Keycode::V, 0xF),
];

pub struct Keyboard {
    pub key_state: [(Keycode, bool); 16]
}

impl Default for Keyboard {
    fn default() -> Self {
        let mut key_state = [(Keycode::Num1, false); 16];
        KEYS.iter()
            .enumerate()
            .for_each(|(i, (k, _))| {
               key_state[i].0 = *k;
            });
        Self {
            key_state
        }
    }
}

pub fn map_key_to_u8(key: Keycode) -> Option<u8> {
    for (keycode, value) in KEYS.iter() {
        if *keycode == key {
            return Some(*value);
        }
    }
    None
}
