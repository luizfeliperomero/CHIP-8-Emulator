mod memory;
mod cpu;
mod display;
use memory::Memory;
use cpu::CPU;
use display::Display;

fn main() {
    let mut memory = Memory::new();
    let rom = "roms/spaceinvaders.ch8";
    match memory.load(rom) {
        Ok(_) => {
            memory.display();
        }
        Err(e) => {
            panic!("{e}: {rom}");
        }
    }
    let display = Display::default();
    let mut cpu = CPU::new(memory, display);
    cpu.run();
}
