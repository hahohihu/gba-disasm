mod types;
mod thumb {
    pub mod msr;
    pub mod addsub;
    pub mod alu;
    pub mod load_store_ext;
    pub mod pcrl;
    pub mod load_addr;
}

use types::*;
use thumb::msr::MoveShiftedRegister;

// Inclusive range - descending, with 0 on the right, similar to the ARM7 datasheet
#[macro_export]
macro_rules! get_bits {
    ($num:expr, $lhs:literal..$rhs:literal) => {{
        assert!($lhs >= $rhs);
        ($num >> $rhs) & ((1 << ($lhs + 1 - $rhs)) - 1)
    }};
    ($num:expr, $lhs:literal..) => {{
        assert!($lhs >= 0);
        $num & ((1 << $lhs) - 1)
    }};
    ($num:expr, ..$rhs:literal) => {{
        assert!($rhs >= 0);
        $num >> $rhs
    }};
}

pub fn get_bit(input: u16, n: u8) -> u8 { 
    if (input & (1 << n)) != 0 { 
        1
    } else {
        0
    }
}

#[derive(Debug, Clone, Copy)]
enum LoHiRegister {
    Lo(Register),
    Hi(u8)
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

