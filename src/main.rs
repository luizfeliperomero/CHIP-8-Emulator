mod memory;
mod cpu;
mod display;
mod keyboard;
use memory::Memory;
use cpu::CPU;
use display::Display;
use keyboard::Keyboard;

fn main() {
    let mut memory = Memory::new();
    let rom = "roms/spaceinvaders.ch8";
    match memory.load(rom) {
        Ok(_) => {
            //memory.display();
        }
        Err(e) => {
            panic!("{e}: {rom}");
        }
    }
    let display = Display::default();
    let keyboard = Keyboard::default();
    let mut cpu = CPU::new(memory, display, keyboard);
    cpu.run();
}
