#[cfg(test)]
use super::register;

/// testing the register constructor
#[test]
fn create_registers() {
    let new_reg = register::Registers::new();
    assert!(
        new_reg
            == register::Registers {
                a: 0,
                b: 0,
                f: register::FlagsRegister(0),
                c: 0,
                d: 0,
                l: 0,
                e: 0,
                h: 0
            }
    )
}

/// testing some bitwise operations on the f flag
#[test]
fn test_flags_reg_values() {
    let mut new_reg = register::Registers::new();
    assert_eq!(new_reg.f.is_carry(), false);
    new_reg.f.set_carry(true);
    new_reg.f.set_substraction(true);
    new_reg.f.set_zero(true);
    new_reg.f.set_half_carry(true);
    assert_eq!(new_reg.f.is_substraction(), true);
    assert_eq!(new_reg.f.is_carry(), true);
    assert_eq!(new_reg.f.is_zero(), true);
    assert_eq!(new_reg.f.is_half_carry(), true);
    new_reg.f.set_carry(false);
    assert_eq!(new_reg.f.is_carry(), false);
    new_reg.f.set_zero(false);
    assert_eq!(new_reg.f.is_zero(), false);
    new_reg.f.set_substraction(false);
    assert_eq!(new_reg.f.is_substraction(), false);
    new_reg.f.set_half_carry(false);
    assert_eq!(new_reg.f.is_half_carry(), false);
}
