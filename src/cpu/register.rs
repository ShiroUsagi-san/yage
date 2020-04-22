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
pub struct Registers {
    pub a: u8,
    pub f: FlagsRegister,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

#[derive(Copy, Clone, PartialEq)]
pub struct FlagsRegister(pub u8);

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        flag.0
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(flag: u8) -> FlagsRegister {
        FlagsRegister(flag)
    }
}

impl fmt::Debug for FlagsRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:b}", self.0);
        writeln!(f, "ZERO FLAG: {}", self.is_zero());
        writeln!(f, "SUBSTRACTION FLAG: {}", self.is_substraction());
        writeln!(f, "HALF CARRY FLAG: {}", self.is_half_carry());
        writeln!(f, "CARRY FLAG: {}", self.is_carry())
    }
}

impl FlagsRegister {
    pub fn new() -> FlagsRegister {
        FlagsRegister(0)
    }
    pub fn is_zero(&self) -> bool {
        self.0 & ZERO_FLAG_MASK != 0
    }
    pub fn is_substraction(&self) -> bool {
        self.0 & SUBSTRACTION_FLAG_MASK != 0
    }
    pub fn is_half_carry(&self) -> bool {
        self.0 & HALF_CARRY_FLAG_MASK != 0
    }
    pub fn is_carry(&self) -> bool {
        self.0 & CARRY_FLAG_MASK != 0
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
        self.0 = if toggle {
            self.0 | offset
        } else {
            self.0 & !offset
        };
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            f: FlagsRegister::new(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }
    // TODO: Avoid repetition on this part later
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8 | self.c as u16) as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xff00) >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8 | u8::from(self.f) as u16) as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xff00) >> 8) as u8;
        self.f = ((value & 0xff) as u8).into();
    }
    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8 | self.e as u16) as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xff00) >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8 | self.l as u16) as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xff00) >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }
}
