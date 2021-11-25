mod types;
mod thumb {
    mod msr;
    mod addsub;
}

use types::*;

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

// TODO
struct CPU {}

trait ThumbInstruction {
    fn execute(&self, cpu: &mut CPU);
}

fn decode_thumb(raw: u16) -> Box<dyn ThumbInstruction> {
    match raw >> 13 {
        0b000 => {},
        0b001 => {},
        0b010 => {},
        0b011 => {},
        0b100 => {},
        0b101 => {},
        0b110 => {},
        0b111 => {},
        _ => unreachable!("DEV ERROR")
    }
    unimplemented!();
}

