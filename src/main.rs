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

const DEFAULT_ROM: &str = "roms/spaceinvaders.ch8";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    debug: bool,
    #[arg(short, long, default_value_t = DEFAULT_ROM.to_string())]
    rom: String,
}

fn main() {
    let args = Args::parse();
    let mut memory = Memory::new();
    match memory.load(args.rom.as_str()) {
        Ok(_) => {
            //memory.display();
        }
        Err(e) => {
            panic!("{e}: {}", args.rom.as_str());
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
