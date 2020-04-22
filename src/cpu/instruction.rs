pub enum Instruction {
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

pub enum ExtendedTarget {
    Arithmetic(ArithmeticTarget),
    Stack(StackTarget),
}

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}
pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}
pub enum LoadType {
    Byte(LoadByteSource, LoadByteTarget),
}
pub enum JumpParameter {
    NZ,
    NC,
    Z,
    C,
    HL,
    NONE,
}

pub enum ConditionnalTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    F,
    HF,
    D8,
}
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    F,
    L,
    H,
    AF,
    HL,
    BC,
    DE,
    SP,
    D8,
}
pub enum StackTarget {
    BC,
    DE,
    AF,
    HL,
    SP,
}
// this enum contains all the possible states to jump
pub enum JumpTest {
    NotZero,
    Zero,
    Carry,
    NotCarry,
    Always,
}

impl Instruction {
    pub fn from_bytes(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::NOP),
            0x10 => Some(Instruction::STOP),
            0xf3 => Some(Instruction::DI),
            0x76 => Some(Instruction::HALT),
            0xfb => Some(Instruction::EI),
            0x0f => Some(Instruction::RRCA),
            0x1f => Some(Instruction::RRA),
            0x37 => Some(Instruction::SFC),
            0x27 => Some(Instruction::DAA),
            0x17 => Some(Instruction::RLA),
            0x07 => Some(Instruction::RLCA),
            0x2f => Some(Instruction::CPL),
            0x3f => Some(Instruction::CCF),

            // RET instructions
            0xc0 => Some(Instruction::RET(JumpParameter::NZ)),
            0xd0 => Some(Instruction::RET(JumpParameter::NC)),
            0xc8 => Some(Instruction::RET(JumpParameter::Z)),
            0xd8 => Some(Instruction::RET(JumpParameter::C)),
            0xc9 => Some(Instruction::RET(JumpParameter::NONE)),

            // RETI instruction
            0xd9 => Some(Instruction::RETI),

            // JP instructions
            0xc2 => Some(Instruction::JP(JumpParameter::NZ)),
            0xd2 => Some(Instruction::JP(JumpParameter::NC)),
            0xca => Some(Instruction::JP(JumpParameter::Z)),
            0xda => Some(Instruction::JP(JumpParameter::C)),
            0xe9 => Some(Instruction::JP(JumpParameter::HL)),
            0xc3 => Some(Instruction::JP(JumpParameter::NONE)),

            // JR instructions
            0x20 => Some(Instruction::JR(JumpParameter::NZ)),
            0x30 => Some(Instruction::JR(JumpParameter::NC)),
            0x18 => Some(Instruction::JR(JumpParameter::NONE)),
            0x28 => Some(Instruction::JR(JumpParameter::Z)),
            0x38 => Some(Instruction::JR(JumpParameter::C)),

            // PUSH instructions
            0xc5 => Some(Instruction::PUSH(StackTarget::BC)),
            0xd5 => Some(Instruction::PUSH(StackTarget::DE)),
            0xe5 => Some(Instruction::PUSH(StackTarget::HL)),
            0xf5 => Some(Instruction::PUSH(StackTarget::AF)),

            // POP instructions
            0xc1 => Some(Instruction::POP(StackTarget::BC)),
            0xd1 => Some(Instruction::POP(StackTarget::DE)),
            0xe1 => Some(Instruction::POP(StackTarget::HL)),
            0xf1 => Some(Instruction::POP(StackTarget::AF)),

            // CALL instructions
            0xc4 => Some(Instruction::CALL(JumpParameter::NZ)),
            0xd4 => Some(Instruction::CALL(JumpParameter::NC)),
            0xcc => Some(Instruction::CALL(JumpParameter::Z)),
            0xdc => Some(Instruction::CALL(JumpParameter::C)),
            0xcd => Some(Instruction::CALL(JumpParameter::NONE)),

            // RST instructions
            // This synthax needs (for now) nightly build of rust (see https://github.com/rust-lang/rust/issues/54883)
            n @ (0xc7 | 0xd7 | 0xe7 | 0xf7 | 0xcf | 0xdf | 0xef | 0xff) => {
                Some(Instruction::RST(n))
            }

            // CP instructions
            0xb8 => Some(Instruction::CP(ArithmeticTarget::B)),
            0xb9 => Some(Instruction::CP(ArithmeticTarget::C)),
            0xba => Some(Instruction::CP(ArithmeticTarget::D)),
            0xbb => Some(Instruction::CP(ArithmeticTarget::E)),
            0xbc => Some(Instruction::CP(ArithmeticTarget::H)),
            0xbd => Some(Instruction::CP(ArithmeticTarget::L)),
            0xbe => Some(Instruction::CP(ArithmeticTarget::HL)),
            0xbf => Some(Instruction::CP(ArithmeticTarget::A)),

            // ADD instructions
            0x80 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::B,
            ))),
            0x81 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::C,
            ))),
            0x82 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::D,
            ))),
            0x83 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::E,
            ))),
            0x84 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::H,
            ))),
            0x85 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::L,
            ))),
            0x86 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::HL,
            ))),
            0x87 => Some(Instruction::ADD(ExtendedTarget::Arithmetic(
                ArithmeticTarget::A,
            ))),
            0x09 => Some(Instruction::ADD(ExtendedTarget::Stack(StackTarget::BC))),
            0x19 => Some(Instruction::ADD(ExtendedTarget::Stack(StackTarget::DE))),
            0x29 => Some(Instruction::ADD(ExtendedTarget::Stack(StackTarget::HL))),
            0x39 => Some(Instruction::ADD(ExtendedTarget::Stack(StackTarget::SP))),

            _ => None,
        }
    }
}
