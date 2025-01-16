use crate::memory::Memory;

#[derive(Debug)]
enum Instruction {
    Jump(u16),
    SkipEqual(),
    Load()
}

pub struct CPU {
    v: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    memory: Memory
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
       Self {
           v: [0; 16],
           i: 0,
           pc: 0x200,
           sp: 0,
           memory,
       } 
    }
    pub fn run(&mut self) {
        loop {
            let lhs = self.memory.memory[self.pc as usize];
            let rhs = self.memory.memory[(self.pc + 1) as usize];
            dbg!(self.decode(lhs, rhs));
        }
    }
    fn decode(&mut self, lhs: u8, rhs: u8) -> Instruction {
        let op = lhs >> 4;
        match op {
            1 => {
                let address = (((lhs & 0x0F) as u16) << 8) | rhs as u16;
                self.pc = address;
                Instruction::Jump(address)
            }
            5 => {
                let x = lhs & 0x0F;
                let y = rhs >> 4;
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 4;
                } else {
                    self.increment_pc();
                }
                Instruction::SkipEqual()
            }
            6 => {
                let x = lhs & 0x0F;
                self.v[x as usize] = rhs;
                self.increment_pc();
                Instruction::Load()
            }
            0xA => {
                let address = (((lhs & 0x0F) as u16) << 8) | rhs as u16;
                self.i = address;
                self.increment_pc();
                Instruction::Load()
            }
            _ => {
                println!("UNIMPLEMENTED OPCODE: {:X?}", op);
                unimplemented!()
            }
        }
    }
    fn increment_pc(&mut self) {
        self.pc += 2;
    }
}
