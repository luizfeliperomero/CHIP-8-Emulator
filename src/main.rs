mod cpu;
mod display;
mod keyboard;
mod memory;
use cpu::CPU;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;

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
    let sdl_context = sdl2::init().unwrap();
    let display = Display::new(&sdl_context);
    let keyboard = Keyboard::new();
    let mut cpu = CPU::new(memory, display, keyboard);
    cpu.run(&sdl_context);
}
