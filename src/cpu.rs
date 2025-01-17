use crate::memory::Memory;
use crate::display::Display;
use crate::display::HEIGHT;
use crate::display::WIDTH;
use crate::keyboard::{KEYS, map_key_to_u8};
use sdl2::event::Event;

#[derive(Debug)]
enum Instruction {
    Jump(u16),
    SkipEqual(),
    Load(),
    Display(),
    Add()
}

pub struct CPU {
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    memory: Memory,
    display: Display
}

impl CPU {
    pub fn new(memory: Memory, display: Display) -> Self {
       Self {
           v: [0; 16],
           i: 0,
           pc: 0x200,
           sp: 0,
           dt: 0,
           st: 0,
           memory,
           display
       } 
    }
    pub fn run(&mut self) {
        loop {
            let lhs = self.memory.memory[self.pc as usize];
            let rhs = self.memory.memory[(self.pc + 1) as usize];
            dbg!(self.decode(lhs, rhs));
            self.display.draw();
        }
    }
    fn decode(&mut self, lhs: u8, rhs: u8) -> Instruction {
        let op = Self::get_leftmost_nibble(lhs);
        match op {
            0x1 => {
                let address = (((Self::get_rightmost_nibble(lhs)) as u16) << 8) | rhs as u16;
                self.pc = address;
                Instruction::Jump(address)
            }
            0x3 => {
                let x = Self::get_rightmost_nibble(lhs);
                if self.v[x as usize] == rhs {
                    self.pc += 4;
                } else {
                    self.increment_pc();
                }
                Instruction::SkipEqual()
            }
            0x5 => {
                let x = Self::get_rightmost_nibble(lhs);
                let y = Self::get_leftmost_nibble(rhs);
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.increment_pc();
                }
                Instruction::SkipEqual()
            }
            0x6 => {
                let x = Self::get_rightmost_nibble(lhs);
                self.v[x as usize] = rhs;
                self.increment_pc();
                Instruction::Load()
            }
            0x7 => {
                let x = Self::get_rightmost_nibble(lhs);
                self.v[x as usize] += rhs;
                self.increment_pc();
                Instruction::Add()
            }
            0xA => {
                let address = (((Self::get_rightmost_nibble(lhs)) as u16) << 8) | rhs as u16;
                self.i = address;
                self.increment_pc();
                Instruction::Load()
            }
            0xD => {
                let n = Self::get_rightmost_nibble(rhs);
                let x = Self::get_rightmost_nibble(lhs);
                let y = Self::get_leftmost_nibble(rhs);
                for index in 0..n {
                     for bit in 0..8 {
                        let sprite_bit = (self.memory.memory[self.i as usize + index as usize] >> (7 - bit)) & 1 != 0;
                        let screen_x = (self.v[x as usize] as usize + bit) % (WIDTH as usize);
                        let screen_y = (self.v[y as usize] as usize + (index as usize)) % (HEIGHT as usize);
                        let display_bit = self.display.pixels[screen_x][screen_y]; 
                        self.display.pixels[screen_x][screen_y] ^= sprite_bit;
                        if display_bit && sprite_bit {
                            self.v[15] = 1;
                        } else {
                            self.v[15] = 0;
                        }
                    }
                }
                self.increment_pc();
                Instruction::Display()
            }
            0xF => {
                match rhs {
                    0x7 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        self.v[x as usize] = self.dt;
                        self.increment_pc();
                        Instruction::Load()
                    }
                    0xA => {
                        loop {
                            for event in self.display.event_pump.poll_iter() {
                                match event {
                                    Event::KeyDown { keycode: Some(key), .. } => {
                                        let x = Self::get_leftmost_nibble(lhs); 
                                        self.v[x as usize] = map_key_to_u8(key).unwrap();
                                        self.increment_pc();
                                        return Instruction::Load();
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                    0x15 => {
                        let x = Self::get_leftmost_nibble(lhs); 
                        self.dt = self.v[x as usize];
                        self.increment_pc();
                        Instruction::Load()
                    }
                    0x18 => {
                        let x = Self::get_leftmost_nibble(lhs); 
                        self.st = self.v[x as usize];
                        self.increment_pc();
                        Instruction::Load()
                    }
                    0x1E => {
                        let x = Self::get_leftmost_nibble(lhs); 
                        self.i += self.v[x as usize] as u16;
                        self.increment_pc();
                        Instruction::Add()
                    }
                    _ => {
                        unimplemented!("UNIMPLEMENTED OPCODE: {:X?}", op)
                    }
                }
            }
            _ => {
                unimplemented!("UNIMPLEMENTED OPCODE: {:X?}", op)
            }
        }
    }
    fn get_rightmost_nibble(n: u8) -> u8 {
        n & 0x0F
    }
    fn get_leftmost_nibble(n: u8) -> u8 {
        n >> 4 
    }
    fn increment_pc(&mut self) {
        self.pc += 2;
    }
}
