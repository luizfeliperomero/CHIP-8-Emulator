use std::fs;
const MEMORY_SIZE: usize = 0x1000;
const STACK_SIZE: usize = 0x10;
const ROM_START: usize = 0x200;
pub struct Memory {
    pub memory: [u8; MEMORY_SIZE],
    rom_size: usize,
    pub stack: [u16; STACK_SIZE] 
}
impl Memory {
    pub fn new() -> Self {
        Self{
            memory: [0; MEMORY_SIZE],
            rom_size: 0,
            stack: [0; STACK_SIZE],
        }
    }
    pub fn load(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error + 'static>>{
        self.load_sprites();
        let temp = fs::read(file_path)?;
        self.rom_size = temp.len();
        temp.iter()
            .enumerate()
            .for_each(|(i, b)| {
                self.memory[ROM_START + i] = *b;
            });
        Ok(())
    }
    pub fn load_sprites(&mut self) {
        let sprites = [
                      0xF0, 0x90, 0x90, 0x90, 0xF0,
                      0x20, 0x60, 0x20, 0x20, 0x70,
                      0xF0, 0x10, 0xF0, 0x80, 0xF0,
                      0xF0, 0x10, 0xF0, 0x10, 0xF0,
                      0x90, 0x90, 0xF0, 0x10, 0x10,
                      0xF0, 0x80, 0xF0, 0x10, 0xF0,
                      0xF0, 0x80, 0xF0, 0x90, 0xF0,
                      0xF0, 0x10, 0x20, 0x40, 0x40,
                      0xF0, 0x90, 0xF0, 0x90, 0xF0,
                      0xF0, 0x90, 0xF0, 0x10, 0xF0,
                      0xF0, 0x90, 0xF0, 0x90, 0x90,
                      0xE0, 0x90, 0xE0, 0x90, 0xE0,
                      0xF0, 0x80, 0x80, 0x80, 0xF0,
                      0xE0, 0x90, 0x90, 0x90, 0xE0,
                      0xF0, 0x80, 0xF0, 0x80, 0xF0,
                      0xF0, 0x80, 0xF0, 0x80, 0x80
                    ];
        sprites.into_iter()
            .enumerate()
            .for_each(|(i, n)| self.memory[i] = n);
    }
    pub fn display(&self) {
        self.memory.chunks(2)
            .skip(ROM_START / 2)
            .take(self.rom_size / 2)
            .enumerate()
            .for_each(|(i,n)| {
                print!("{:02X?}: ", ROM_START + i);
                n.iter()
                    .for_each(|e| {
                        print!("{:02X?}", e);
                    });
                print!(" ");
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sprites_were_loaded_correctly() {
        let mut memory = Memory::new();
        memory.load_sprites();
        let sprites = [
                      0xF0, 0x90, 0x90, 0x90, 0xF0,
                      0x20, 0x60, 0x20, 0x20, 0x70,
                      0xF0, 0x10, 0xF0, 0x80, 0xF0,
                      0xF0, 0x10, 0xF0, 0x10, 0xF0,
                      0x90, 0x90, 0xF0, 0x10, 0x10,
                      0xF0, 0x80, 0xF0, 0x10, 0xF0,
                      0xF0, 0x80, 0xF0, 0x90, 0xF0,
                      0xF0, 0x10, 0x20, 0x40, 0x40,
                      0xF0, 0x90, 0xF0, 0x90, 0xF0,
                      0xF0, 0x90, 0xF0, 0x10, 0xF0,
                      0xF0, 0x90, 0xF0, 0x90, 0x90,
                      0xE0, 0x90, 0xE0, 0x90, 0xE0,
                      0xF0, 0x80, 0x80, 0x80, 0xF0,
                      0xE0, 0x90, 0x90, 0x90, 0xE0,
                      0xF0, 0x80, 0xF0, 0x80, 0xF0,
                      0xF0, 0x80, 0xF0, 0x80, 0x80
                    ];
        memory.memory.iter()
                     .take(sprites.len())
                     .zip(sprites.iter())
                     .for_each(|(s, m)| {
                         if s != m {
                             panic!();
                         }
                     });
    }
}
