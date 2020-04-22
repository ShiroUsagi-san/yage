use crate::cpu::register::Registers;

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
