use std::fs;
const MEMORY_SIZE: usize = 0x1000;
const ROM_START: usize = 0x200;
pub struct Memory {
    pub memory: [u8; MEMORY_SIZE],
    rom_size: usize
}
impl Memory {
    pub fn new() -> Self {
        Self{
            memory: [0; MEMORY_SIZE],
            rom_size: 0
        }
    }
    pub fn load(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error + 'static>>{
        let temp = fs::read(file_path)?;
        self.rom_size = temp.len();
        temp.iter()
            .enumerate()
            .for_each(|(i, b)| {
                self.memory[ROM_START + i] = *b;
            });
        Ok(())
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
