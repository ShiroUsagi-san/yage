#[cfg(test)]
use super::register;

/// testing the register constructor
#[test]
fn create_registers() {
    let new_reg = register::CpuRegisters::new();
    assert_eq!(new_reg, register::CpuRegisters([0; 8]))
}

/// testing some bitwise operations on the f flag
#[test]
fn test_flags_reg_values() {
    let mut new_reg = register::CpuRegisters::new();
    assert_eq!(new_reg.is_carry(), false);
    new_reg.set_carry(true);
    new_reg.set_substraction(true);
    new_reg.set_zero(true);
    new_reg.set_half_carry(true);
    assert_eq!(new_reg.is_substraction(), true);
    assert_eq!(new_reg.is_carry(), true);
    assert_eq!(new_reg.is_zero(), true);
    assert_eq!(new_reg.is_half_carry(), true);
    new_reg.set_carry(false);
    assert_eq!(new_reg.is_carry(), false);
    new_reg.set_zero(false);
    assert_eq!(new_reg.is_zero(), false);
    new_reg.set_substraction(false);
    assert_eq!(new_reg.is_substraction(), false);
    new_reg.set_half_carry(false);
    assert_eq!(new_reg.is_half_carry(), false);
}
