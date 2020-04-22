use crate::cpu::{instruction::ArithmeticTarget, instruction::Instruction, register::Registers};

pub struct Cpu {
    registers: Registers,
    flags: u8,
    memory: MemoryBus,
    sp: u8,
    pc: u16,
    is_halted: bool,
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}
impl Cpu {
    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }
    fn step(&mut self) {
        if self.is_halted {
            return;
        }
        let instruction_byte = self.memory.read_byte(self.pc);
        let next_pc = if let Some(instruction) = Instruction::from_bytes(instruction_byte) {
            self.execute(instruction)
        } else {
            // improve that to avoid crash
            panic!("Unkown instruction found at 0x{:X}", instruction_byte);
        };
        self.pc = next_pc;
    }
    fn execute(&mut self, ins: Instruction) -> u16 {
        match ins {
            Instruction::ADD(target) => match target {
                /*
                ArithmeticTarget::C => {
                    let val = self.registers.c;
                    let new_val = self.add(val);
                    self.registers.a = new_val;
                    self.pc.wrapping_add(1)
                }*/
                _ => {
                    unimplemented!();
                }
            },
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            _ => {
                unimplemented!();
            }
        }
    }
    fn add(&mut self, val: u8) -> u8 {
        let (new_val, did_overflow) = self.registers.a.overflowing_add(val);
        self.registers.f.set_zero(new_val == 0);
        self.registers.f.set_substraction(false);
        self.registers.f.set_carry(did_overflow);
        self.registers
            .f
            .set_half_carry((self.registers.a & 0xf) + (val & 0xf) > 0xf);
        new_val
    }
}
