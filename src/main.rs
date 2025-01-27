mod cpu;
mod debugger;
mod display;
mod keyboard;
mod memory;
use clap::Parser;
use cpu::CPU;
use display::Display;
use keyboard::Keyboard;
use memory::Memory;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let mut memory = Memory::new();
    let rom = "roms/tetris.ch8";
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
    if args.debug {
        cpu.run_debug(&sdl_context);
    } else {
        cpu.run(&sdl_context);
    }
}
