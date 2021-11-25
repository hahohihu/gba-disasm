mod types;
mod thumb {
    pub mod msr;
}

use types::*;
use thumb::msr::MoveShiftedRegister;

#[derive(Debug, Clone, Copy)]
enum LoHiRegister {
    Lo(Register),
    Hi(u8)
}

#[derive(Debug, Clone, Copy)]
enum AddSubtractOpCode {
    ADD,
    SUB
}

#[derive(Debug, Clone, Copy)]
struct AddSubtract {
    op: AddSubtractOpCode,
    roi: RegisterOrImmediate,
    src: Register,
    dest: Register
}

#[derive(Debug, Clone, Copy)]
enum MoveCompareAddSubtractImmediateOpCode {
    MOV,
    CMP,
    ADD,
    SUB
}

#[derive(Debug, Clone, Copy)]
struct MoveCompareAddSubtractImmediate {
    op: MoveCompareAddSubtractImmediateOpCode,
    dest: Register,
    offset: Immediate
}

#[derive(Debug, Clone, Copy)]
enum AluOperationsOpCode {
    AND,
    EOR,
    LSL,
    LSR,
    ASR,
    ADC,
    SBC,
    ROR,
    TST,
    NEG,
    CMP,
    CMN,
    ORR,
    MUL,
    BIC,
    MVN
}

#[derive(Debug, Clone, Copy)]
struct AluOperations {
    op: AluOperationsOpCode,
    src: Register,
    dest: Register
}

#[derive(Debug, Clone, Copy)]
enum HiRegisterOpCode {
    ADD,
    CMP,
    MOV,
    BX
}

#[derive(Debug, Clone, Copy)]
struct HiRegisterOp {
    opcode: HiRegisterOpCode,
    src: LoHiRegister,
    dst: LoHiRegister
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThumbInstruction {
    MSR(MoveShiftedRegister)
}

fn decode_thumb(raw: u16) -> ThumbInstruction {
    match raw >> 13 {
        0b000 => {
            match raw >> 11 {
                0b11 => unimplemented!(),
                _ => ThumbInstruction::MSR(MoveShiftedRegister::from(raw))
            }
        },
        0b001 => unimplemented!(),
        0b010 => unimplemented!(),
        0b011 => unimplemented!(),
        0b100 => unimplemented!(),
        0b101 => unimplemented!(),
        0b110 => unimplemented!(),
        0b111 => unimplemented!(),
        _ => unreachable!("DEV ERROR")
    }
}

