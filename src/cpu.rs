use crate::display::DisplayTrait;
use crate::display::HEIGHT;
use crate::display::WIDTH;
use crate::keyboard::{map_key_to_u8, Keyboard, KEYS};
use crate::memory::Memory;
use sdl2::Sdl;
use rand::Rng;
use sdl2;
use sdl2::event::Event;

#[derive(Debug)]
enum Instruction {
    Jump(u16),
    SkipEqual,
    Load,
    Display,
    Add,
    Call,
    Or,
    And,
    XOR,
    Sub,
    SHR,
    SubN,
    SHL,
    SNE,
    RND,
    SKP,
    SKNP,
    CLS,
    RET,
}

pub struct CPU<D: DisplayTrait> {
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    dt: u8,
    st: u8,
    memory: Memory,
    display: D,
    keyboard: Keyboard,
    waiting_key: bool
}

impl<D: DisplayTrait> CPU<D> {
    pub fn new(memory: Memory, display: D, keyboard: Keyboard) -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
            memory,
            display,
            keyboard,
            waiting_key: false,
        }
    }
    pub fn run(&mut self, sdl_context: &Sdl) {
        loop {
            self.keyboard.update(sdl_context);
            if self.waiting_key && self.keyboard.is_any_pressed() {
                self.increment_pc();
                self.waiting_key = false;
            }
            if self.display.draw() {
                let lhs = self.memory.memory[self.pc as usize];
                let rhs = self.memory.memory[(self.pc + 1) as usize];
                self.decode(lhs, rhs);
            }
        }
    }
    fn decode(&mut self, lhs: u8, rhs: u8) -> Instruction {
        let op = Self::get_leftmost_nibble(lhs);
        match op {
            0x0 => match rhs {
                0xE0 => {
                    self.display.clear();
                    self.increment_pc();
                    Instruction::CLS
                }
                0xEE => {
                    // TODO (luizf): Different from Cowgod's reference
                    if self.sp == 0 {
                        panic!("Stack Underflow!")
                    }
                    self.sp -= 1;
                    self.pc = self.memory.stack[self.sp as usize];
                    self.increment_pc();
                    Instruction::RET
                }
                _ => {
                    unimplemented!("Unimplemented OPCODE FOR 0");
                }
            },
            0x1 => {
                let address = (((Self::get_rightmost_nibble(lhs)) as u16) << 8) | rhs as u16;
                self.pc = address;
                Instruction::Jump(address)
            }
            0x2 => {
                // TODO (luizf): Different from Cowgod's reference
                let address = (((Self::get_rightmost_nibble(lhs)) as u16) << 8) | rhs as u16;
                if self.sp == 15 {
                    panic!("Stack Overflow!")
                }
                self.memory.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = address;
                Instruction::Call
            }
            0x3 => {
                let x = Self::get_rightmost_nibble(lhs);
                if self.v[x as usize] == rhs {
                    self.skip_next_instruction();
                } else {
                    self.increment_pc();
                }
                Instruction::SkipEqual
            }
            0x4 => {
                let x = Self::get_rightmost_nibble(lhs);
                if self.v[x as usize] != rhs {
                    self.skip_next_instruction();
                } else {
                    self.increment_pc();
                }
                Instruction::SNE
            }
            0x5 => {
                let x = Self::get_rightmost_nibble(lhs);
                let y = Self::get_leftmost_nibble(rhs);
                if self.v[x as usize] == self.v[y as usize] {
                    self.skip_next_instruction();
                } else {
                    self.increment_pc();
                }
                Instruction::SkipEqual
            }
            0x6 => {
                let x = Self::get_rightmost_nibble(lhs);
                self.v[x as usize] = rhs;
                self.increment_pc();
                Instruction::Load
            }
            0x7 => {
                let x = Self::get_rightmost_nibble(lhs);
                // TODO (luizf): Potentially incorrect code
                self.v[x as usize] = self.v[x as usize].wrapping_add(rhs);
                self.increment_pc();
                Instruction::Add
            }
            0x8 => {
                let n = Self::get_rightmost_nibble(rhs);
                match n {
                    0x0 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        let y = Self::get_leftmost_nibble(rhs);
                        self.v[x as usize] = self.v[y as usize];
                        self.increment_pc();
                        Instruction::Load
                    }
                    0x1 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        let y = Self::get_leftmost_nibble(rhs);
                        self.v[x as usize] |= self.v[y as usize];
                        self.increment_pc();
                        Instruction::Or
                    }
                    0x2 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        let y = Self::get_leftmost_nibble(rhs);
                        self.v[x as usize] &= self.v[y as usize];
                        self.increment_pc();
                        Instruction::And
                    }
                    0x3 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        let y = Self::get_leftmost_nibble(rhs);
                        self.v[x as usize] ^= self.v[y as usize];
                        self.increment_pc();
                        Instruction::XOR
                    }
                    0x4 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        let y = Self::get_leftmost_nibble(rhs);
                        let vx = self.v[x as usize] as u16 + self.v[y as usize] as u16;
                        self.v[x as usize] = vx as u8;
                        self.v[0xF] = if vx > 0xFF { 1 } else { 0 };
                        self.increment_pc();
                        Instruction::Add
                    }
                    0x5 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        let y = Self::get_leftmost_nibble(rhs);
                        self.v[0xF] = if self.v[x as usize] > self.v[y as usize] {
                            1
                        } else {
                            0
                        };
                        self.v[x as usize] -= self.v[y as usize];
                        self.increment_pc();
                        Instruction::Sub
                    }
                    0x6 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        self.v[0xF] = self.v[x as usize] & 1;
                        self.v[x as usize] >>= 1;
                        self.increment_pc();
                        Instruction::SHR
                    }
                    0x7 => {
                        let x = Self::get_rightmost_nibble(lhs);
                        let y = Self::get_leftmost_nibble(rhs);
                        self.v[0xF] = if self.v[y as usize] > self.v[x as usize] {
                            1
                        } else {
                            0
                        };
                        self.v[x as usize] = self.v[y as usize] - self.v[x as usize];
                        self.increment_pc();
                        Instruction::SubN
                    }
                    0xE => {
                        let x = Self::get_rightmost_nibble(lhs);
                        self.v[0xF] = self.v[x as usize] >> 7 & 1;
                        self.v[x as usize] <<= 1;
                        self.increment_pc();
                        Instruction::SHL
                    }
                    _ => {
                        unimplemented!("UNIMPLEMENTED OPCODE: {:X?}", op)
                    }
                }
            }
            0x9 => {
                let x = Self::get_rightmost_nibble(lhs);
                let y = Self::get_leftmost_nibble(rhs);
                if self.v[x as usize] != self.v[y as usize] {
                    self.skip_next_instruction();
                } else {
                    self.increment_pc();
                }
                Instruction::SNE
            }
            0xA => {
                let address = (((Self::get_rightmost_nibble(lhs)) as u16) << 8) | rhs as u16;
                self.i = address;
                self.increment_pc();
                Instruction::Load
            }
            0xB => {
                let address = ((((Self::get_rightmost_nibble(lhs)) as u16) << 8) | rhs as u16)
                    + self.v[0] as u16;
                self.pc = address;
                Instruction::Jump(address)
            }
            0xC => {
                let x = Self::get_rightmost_nibble(lhs);
                let random: u8 = rand::thread_rng().gen();
                self.v[x as usize] = random & rhs;
                self.increment_pc();
                Instruction::RND
            }
            0xD => {
                let n = Self::get_rightmost_nibble(rhs);
                let x = Self::get_rightmost_nibble(lhs);
                let y = Self::get_leftmost_nibble(rhs);
                let mut vy = self.v[y as usize];
                let mut vf_changed = false;
                for sprite_index in self.i..(self.i + n as u16) {
                    let start_index: usize = sprite_index as usize;
                    if vy as usize >= HEIGHT {
                        vy = 0;
                    }
                    let sprite_row = self.memory.memory[start_index];
                    for b in 0..8 {
                        let mut vx = self.v[x as usize];
                        if vx as usize + b as usize >= WIDTH {
                            vx = 0;
                        }
                        let pixel_index: usize =
                            ((vy as usize * WIDTH * 3) + ((vx as usize + b) * 3)) as usize;
                        let old_pixel = self.display.get_pixel(pixel_index);
                        let new_pixel = sprite_row & (0b1000_0000 >> b);
                        let new_pixel = if new_pixel != 0 { 0xFF } else { 0x00 };
                        let new_pixel = new_pixel ^ old_pixel;
                        self.display.set_pixel(pixel_index as usize, new_pixel);
                        self.display.set_pixel(pixel_index + 1, new_pixel);
                        self.display.set_pixel(pixel_index + 2, new_pixel);
                        vf_changed = vf_changed || (old_pixel & new_pixel) != old_pixel;
                    }
                    vy += 1;
                }
                self.v[0xF] = if vf_changed { 1 } else { 0 };
                self.increment_pc();
                Instruction::Display
            }
            0xE => match rhs {
                0x9E => {
                    let x = Self::get_rightmost_nibble(lhs);
                    if self.keyboard.is_pressed(self.v[x as usize]) {
                        self.skip_next_instruction();
                    } else {
                        self.increment_pc();
                    }
                    Instruction::SKP
                }
                0xA1 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    if !self.keyboard.is_pressed(self.v[x as usize]) {
                        self.skip_next_instruction();
                    } else {
                        self.increment_pc();
                    }
                    Instruction::SKNP
                }
                _ => {
                    unimplemented!("Unimplemented E")
                }
            },
            0xF => match rhs {
                0x7 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.v[x as usize] = self.dt;
                    self.increment_pc();
                    Instruction::Load
                }
                0xA => {
                    self.waiting_key = true;
                    return Instruction::Load;
                }
                0x15 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.dt = self.v[x as usize];
                    self.increment_pc();
                    Instruction::Load
                }
                0x18 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.st = self.v[x as usize];
                    self.increment_pc();
                    Instruction::Load
                }
                0x1E => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.i += self.v[x as usize] as u16;
                    self.increment_pc();
                    Instruction::Add
                }
                0x29 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.i = self.v[x as usize] as u16 * 5;
                    self.increment_pc();
                    Instruction::Load
                }
                0x33 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.memory.memory[(self.i + 2) as usize] = self.v[x as usize] % 10;
                    self.memory.memory[(self.i + 1) as usize] = (self.v[x as usize] / 10) % 10;
                    self.memory.memory[self.i as usize] = (self.v[x as usize] / 100) % 10;
                    self.increment_pc();
                    Instruction::Load
                }
                0x55 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.v
                        .iter()
                        .take(x as usize + 1)
                        .enumerate()
                        .for_each(|(i, n)| {
                            self.memory.memory[i + self.i as usize] = *n;
                        });
                    self.increment_pc();
                    Instruction::Load
                }
                0x65 => {
                    let x = Self::get_rightmost_nibble(lhs);
                    self.memory
                        .memory
                        .iter()
                        .skip(self.i as usize)
                        .take(x as usize + 1)
                        .enumerate()
                        .for_each(|(index, n)| {
                            self.v[index] = *n;
                        });
                    self.increment_pc();
                    Instruction::Load
                }
                _ => {
                    unimplemented!("UNIMPLEMENTED OPCODE: {:X?}", op)
                }
            },
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
    fn skip_next_instruction(&mut self) {
        self.pc += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct FakeDisplay {}
    impl DisplayTrait for FakeDisplay {
        fn draw(&mut self) -> bool {
            true
        }
        fn clear(&mut self) {}
        fn get_pixels(&self) -> [u8; WIDTH * HEIGHT * 3] {
            [0; WIDTH * HEIGHT * 3]
        }
        fn set_pixels(&mut self, value: [u8; WIDTH * HEIGHT * 3]) {}
        fn get_pixel(&self, _index: usize) -> u8 {
            0
        }
        fn set_pixel(&mut self, _index: usize, value: u8) {}
    }
    fn cpu() -> CPU<FakeDisplay> {
        CPU::new(Memory::new(), FakeDisplay {}, Keyboard::new())
    }
    #[test]
    // 00EE - RET
    fn test_stack_pop_updates_pc_and_sp() {
        let mut cpu = cpu();
        cpu.sp = 0xf;
        cpu.decode(0x0, 0xEE);
        assert_eq!(cpu.pc, cpu.memory.memory[0xf] as u16);
        assert_eq!(cpu.sp, 0xE);
    }

    #[test]
    // 1nnn - JP addr
    fn should_set_pc_to_nnn() {
        let mut cpu = cpu();
        cpu.decode(0x12, 0xAA);
        assert_eq!(cpu.pc, 0x2AA);
    }
    #[test]
    // 2nnn - CALL addr
    fn test_stack_push_and_jump_to_nnn() {
        let mut cpu = cpu();
        cpu.sp = 1;
        cpu.pc = 0x200;
        cpu.decode(0x21, 0xAA);
        assert_eq!(cpu.memory.stack[cpu.sp as usize], 0x200);
        assert_eq!(cpu.sp, 2);
        assert_eq!(cpu.pc, 0x1AA);
    }
    #[test]
    // 3xkk - SE Vx, byte
    fn should_skip_if_vx_eq_kk() {
        let mut cpu = cpu();
        cpu.pc = 0x200;
        cpu.v[1] = 0xAA;
        cpu.decode(0x31, 0xAA);
        assert_eq!(cpu.pc, 0x204);
    }

    #[test]
    // 4xkk - SNE Vx, byte
    fn should_skip_if_vx_ne_kk() {
        let mut cpu = cpu();
        cpu.pc = 0x200;
        cpu.v[1] = 0xBA;
        cpu.decode(0x41, 0xAA);
        assert_eq!(cpu.pc, 0x204);
        cpu.pc = 0x200;
        cpu.v[1] = 0xAA;
        cpu.decode(0x41, 0xAA);
        assert_ne!(cpu.pc, 0x204);
    }
    #[test]
    // 5xy0 - SE Vx, Vy
    fn should_skip_if_vx_eq_vy() {
        let mut cpu = cpu();
        cpu.pc = 0x200;
        cpu.v[0x1] = 0xBA;
        cpu.v[0xA] = 0xBA;
        cpu.decode(0x51, 0xA0);
        assert_eq!(cpu.pc, 0x204);
        cpu.pc = 0x200;
        cpu.v[0x1] = 0xFA;
        cpu.v[0xA] = 0xBA;
        cpu.decode(0x51, 0xA0);
        assert_eq!(cpu.pc, 0x202);
    }
    #[test]
    // 6xkk - LD Vx, byte
    fn should_load_kk_in_vx() {
        let mut cpu = cpu();
        cpu.decode(0x6A, 0x11);
        assert_eq!(cpu.v[0xA], 0x11);
    }
    #[test]
    // 7xkk - ADD Vx, byte
    fn should_add_vx_and_kk() {
        let mut cpu = cpu();
        let sum = cpu.v[0xA] + 0x1;
        cpu.decode(0x7A, 0x1);
        assert_eq!(cpu.v[0xA], sum);
    }
    #[test]
    // 8xy0 - LD Vx, Vy
    fn should_load_vy_in_vx() {
        let mut cpu = cpu();
        cpu.v[0x1] = 1;
        cpu.v[0xA] = 34;
        cpu.decode(0x8A, 0x10);
        assert_eq!(cpu.v[0x1], cpu.v[0xA]);
    }
    #[test]
    // 8xy1 - OR Vx, Vy
    fn should_or_vx_and_vy() {
        let mut cpu = cpu();
        cpu.v[0xA] = 2;
        cpu.v[0x1] = 10;
        let result = cpu.v[0xA] | cpu.v[0x1];
        cpu.decode(0x8A, 0x11);
        assert_eq!(cpu.v[0xA], result);
    }
    #[test]
    // 8xy2 - AND Vx, Vy
    fn should_and_vx_and_vy() {
        let mut cpu = cpu();
        cpu.v[0xA] = 2;
        cpu.v[0x2] = 10;
        let result = cpu.v[0xA] & cpu.v[0x2];
        cpu.decode(0x8A, 0x22);
        assert_eq!(cpu.v[0xA], result);
    }
    #[test]
    // 8xy3 - XOR Vx, Vy
    fn should_xor_vx_and_vy() {
        let mut cpu = cpu();
        cpu.v[0xA] = 2;
        cpu.v[0x2] = 10;
        let result = cpu.v[0xA] ^ cpu.v[0x2];
        cpu.decode(0x8A, 0x23);
        assert_eq!(cpu.v[0xA], result);
    }
    #[test]
    // 8xy4 - ADD Vx, Vy
    fn should_add_vx_and_vy_and_set_carry() {
        let mut cpu = cpu();
        let x = 0xA;
        let y = 0x2;

        cpu.v[x] = 0xFF;
        cpu.v[y] = 0x01;

        cpu.decode(0x8A, 0x24);

        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.v[x], 0x00);
    }
    #[test]
    // 8xy5 - SUB Vx, Vy
    fn should_subtract_vx_and_vy_and_update_vf() {
        let mut cpu = cpu();
        let x = 0xA;
        let y = 0x2;

        cpu.v[x] = 0xFF;
        cpu.v[y] = 0x01;

        cpu.decode(0x8A, 0x25);

        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.v[x], 0xFE);
    }
    #[test]
    // 8xy6 - SHR Vx {, Vy}
    fn should_shift_right_vx() {
        let mut cpu = cpu();
        let x = 0xA;
        let y = 0x2;
        cpu.v[x] = 1;
        cpu.v[y] = 2;
        cpu.decode(0x8A, 0x26);
        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.v[x], 0);
    }
    #[test]
    // 8xy7 - SUBN Vx, Vy
    fn should_subtract_vx_if_vy_greater() {
        let mut cpu = cpu();
        let x = 0xA;
        let y = 0x2;
        cpu.v[x] = 8;
        cpu.v[y] = 10;
        cpu.decode(0x8A, 0x27);
        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.v[x], 2);
    }
    #[test]
    // 8xyE - SHL Vx {, Vy}
    fn should_shift_left_vx() {
        let mut cpu = cpu();
        let x = 0xA;
        cpu.v[x] = 0x81;
        cpu.decode(0x8A, 0x2E);
        assert_eq!(cpu.v[0xF], 1);
        assert_eq!(cpu.v[x], 0x02);
    }
    #[test]
    // 9xy0 - SNE Vx, Vy
    fn skip_if_vx_ne_vy() {
        let mut cpu = cpu();
        let x = 0xA;
        let y = 0xB;
        cpu.pc = 0x200;
        cpu.v[x] = 123;
        cpu.v[y] = 42;
        cpu.decode(0x9A, 0xB0);
        assert_eq!(cpu.pc, 0x204);
    }
    #[test]
    // Annn - LD I, addr
    fn should_load_nnn_in_i() {
        let mut cpu = cpu();
        cpu.decode(0xAB, 0x22);
        assert_eq!(cpu.i, 0xB22);
    }
    #[test]
    // Bnnn - JP V0, addr
    fn should_jump_to_nnn_plus_v0() {
        let mut cpu = cpu();
        cpu.decode(0xB1, 0x42);
        assert_eq!(cpu.pc, 0x142);
    }
    #[test]
    // Fx07 - LD Vx, DT
    fn should_load_dt_in_vx() {
        let mut cpu = cpu();
        cpu.dt = 42;
        cpu.decode(0xF1, 0x07);
        assert_eq!(cpu.v[1], 42);
    }
    #[test]
    // Fx15 - LD DT, Vx
    fn should_load_vx_in_dt() {
        let mut cpu = cpu();
        cpu.v[1] = 42;
        cpu.decode(0xF1, 0x15);
        assert_eq!(cpu.dt, 42);
    }
    #[test]
    // Fx18 - LD ST, Vx
    fn should_load_vc_in_st() {
        let mut cpu = cpu();
        cpu.v[1] = 42;
        cpu.decode(0xF1, 0x18);
        assert_eq!(cpu.st, 42);
    }
    #[test]
    // Fx1E - ADD I, Vx
    fn should_add_i_and_vx() {
        let mut cpu = cpu();
        cpu.v[1] = 42;
        cpu.i = 1;
        cpu.decode(0xF1, 0x1E);
        assert_eq!(cpu.i, 43);
    }
    #[test]
    // Fx29 - LD F, Vx
    fn should_set_i_to_location_of_sprite_for_vx() {
        let mut cpu = cpu();
        cpu.v[5] = 2;
        cpu.memory.load_sprites();
        cpu.decode(0xF5, 0x29);
        assert_eq!(cpu.i, 10);
    }
    #[test]
    // Fx33 - LD B, Vx
    fn should_store_bcd_representation_of_vx() {
        let mut cpu = cpu();
        cpu.v[5] = 152;
        cpu.decode(0xF5, 0x33);
        assert_eq!(cpu.memory.memory[cpu.i as usize], 1);
        assert_eq!(cpu.memory.memory[cpu.i as usize + 1], 5);
        assert_eq!(cpu.memory.memory[cpu.i as usize + 2], 2);
    }
    #[test]
    // Fx55 - LD [I], Vx
    fn should_store_registers_v0_through_vx_in_memory_starting_at_i() {
        let mut cpu = cpu();
        cpu.v[0] = 1;
        cpu.v[1] = 2;
        cpu.v[2] = 3;
        cpu.i = 0x200;
        cpu.decode(0xF2, 0x55);
        assert_eq!(cpu.memory.memory[cpu.i as usize], 1);
        assert_eq!(cpu.memory.memory[cpu.i as usize + 1], 2);
        assert_eq!(cpu.memory.memory[cpu.i as usize + 2], 3);
    }
    #[test]
    // Fx65 - LD Vx, [I]
    fn should_read_v0_through_vx_starting_at_i() {
        let mut cpu = cpu();
        cpu.i = 0x200;
        cpu.memory.memory[cpu.i as usize] = 2;
        cpu.memory.memory[cpu.i as usize + 1] = 3;
        cpu.memory.memory[cpu.i as usize + 2] = 4;
        cpu.decode(0xF2, 0x65);
        assert_eq!(cpu.v[0], 2);
        assert_eq!(cpu.v[1], 3);
        assert_eq!(cpu.v[2], 4);
    }
}
