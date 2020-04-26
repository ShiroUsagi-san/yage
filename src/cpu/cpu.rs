use crate::cpu::register::CpuRegisters;

pub struct Cpu {
    registers: CpuRegisters,
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
}
