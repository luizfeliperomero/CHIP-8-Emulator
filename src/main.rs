mod memory;
use memory::Memory;
mod cpu;
use cpu::CPU;

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
    let mut cpu = CPU::new(memory);
    cpu.run();
}
