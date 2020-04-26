use std::fmt;

const ZERO_FLAG_MASK: u8 = 0b1000;
const SUBSTRACTION_FLAG_MASK: u8 = 0b0100;
const HALF_CARRY_FLAG_MASK: u8 = 0b0010;
const CARRY_FLAG_MASK: u8 = 0b0001;

const ZERO_FLAG_SHIFT: u8 = 3;
const SUBSTRACTION_FLAG_SHIFT: u8 = 2;
const HALF_CARRY_FLAG_SHIFT: u8 = 1;
const CARRY_FLAG_SHIFT: u8 = 0;

#[derive(PartialEq, Debug)]
pub struct CpuRegisters(pub [u8; 8]);

pub enum Register {
    A = 0,
    F = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7,
}
pub enum ExtendRegister {
    AF,
    BC,
    DE,
    HL,
}
impl CpuRegisters {
    pub fn new() -> Self {
        CpuRegisters([0; 8])
    }

    pub fn get_reg(&self, reg: Register) -> u8 {
        self.0[reg as usize]
    }

    pub fn get_extend_reg(&self, reg: ExtendRegister) -> u16 {
        match reg {
            ExtendRegister::AF => self.get_shared(Register::A, Register::F),
            ExtendRegister::BC => self.get_shared(Register::B, Register::C),
            ExtendRegister::DE => self.get_shared(Register::D, Register::E),
            ExtendRegister::HL => self.get_shared(Register::H, Register::L)
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0[1] & ZERO_FLAG_MASK != 0
    }

    pub fn is_substraction(&self) -> bool {
        self.0[1] & SUBSTRACTION_FLAG_MASK != 0
    }

    pub fn is_half_carry(&self) -> bool {
        self.0[1] & HALF_CARRY_FLAG_MASK != 0
    }

    pub fn is_carry(&self) -> bool {
        self.0[1] & CARRY_FLAG_MASK != 0
    }

    pub fn set_zero(&mut self, toggle: bool) {
        let offset = 0b1 << ZERO_FLAG_SHIFT;
        &self.toggle_flag(toggle, offset);
    }

    pub fn set_substraction(&mut self, toggle: bool) {
        let offset = 0b1 << SUBSTRACTION_FLAG_SHIFT;
        &self.toggle_flag(toggle, offset);
    }

    pub fn set_half_carry(&mut self, toggle: bool) {
        let offset = 0b1 << HALF_CARRY_FLAG_SHIFT;
        &self.toggle_flag(toggle, offset);
    }

    pub fn set_carry(&mut self, toggle: bool) {
        let offset = 0b1 << CARRY_FLAG_SHIFT;
        &self.toggle_flag(toggle, offset);
    }

    fn get_shared(&self, reg1: Register, reg2: Register) -> u16 {
        (self.0[reg1 as usize] << 8) as u16 | self.0[reg2 as usize] as u16
    }

    fn toggle_flag(&mut self, toggle: bool, offset: u8) {
        self.0[1] = if toggle {
            self.0[1] | offset
        } else {
            self.0[1] & !offset
        };
    }
}
