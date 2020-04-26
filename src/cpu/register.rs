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
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
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
        match reg {
            Register::A => self.0[0],
            Register::F => self.0[1],
            Register::B => self.0[2],
            Register::C => self.0[3],
            Register::D => self.0[4],
            Register::E => self.0[5],
            Register::H => self.0[6],
            Register::L => self.0[7],
        }
    }
    pub fn get_extend_reg(&self, reg: ExtendRegister) -> u16 {
        match reg {
            ExtendRegister::AF => (self.0[0] << 8) as u16 | self.0[1] as u16,
            ExtendRegister::BC => (self.0[2] << 8) as u16 | self.0[3] as u16,
            ExtendRegister::DE => (self.0[4] << 8) as u16 | self.0[5] as u16,
            ExtendRegister::HL => (self.0[6] << 8) as u16 | self.0[7] as u16,
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

    fn toggle_flag(&mut self, toggle: bool, offset: u8) {
        self.0[1] = if toggle {
            self.0[1] | offset
        } else {
            self.0[1] & !offset
        };
    }
}
