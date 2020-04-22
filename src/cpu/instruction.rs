use crate::cpu::cpu::Cpu;

/*
pub enum Instructions {
    NOP,
    HALT,
    EI,
    STOP,
    RLCA,
    RLA,
    RRCA,
    RRA,
    DI,
    SFC,
    RETI,
    DAA,
    CPL,
    CCF,
    SBC(ArithmeticTarget),
    RET(JumpParameter),
    ADD(ExtendedTarget),
    SUB(ArithmeticTarget),
    MUL(ArithmeticTarget),
    DIV(ArithmeticTarget),
    RST(u8),
    DEC(ArithmeticTarget),
    INT(ArithmeticTarget),
    CALL(JumpParameter),
    PUSH(StackTarget),
    POP(StackTarget),
    JR(JumpParameter),
    JP(JumpParameter),
    LD(LoadType),
    LDH,
    OR(ConditionnalTarget),
    AND(ConditionnalTarget),
    ADC(ArithmeticTarget),
    CP(ArithmeticTarget),
    XOR(ArithmeticTarget),
}
*/

// this enum contains all the possible states to jump
pub enum JumpTest {
    NotZero,
    Zero,
    Carry,
    NotCarry,
    Always,
}

/// The Executable trait is implemented for each instruction type
pub trait Executable {
    fn exec(cpu: &mut Cpu) -> u8;
}
