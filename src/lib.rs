#![allow(clippy::upper_case_acronyms)]
mod types;
mod thumb;

use thumb::msr::MoveShiftedRegister;
use thumb::op_immediate::OpImmediate;
use thumb::hi_op::HiOp;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThumbInstruction {
    MSR(MoveShiftedRegister),
    OpImmediate(OpImmediate),
    HiOp(HiOp)
}

pub fn decode_thumb(raw: u16) -> ThumbInstruction {
    match get_bits!(raw, ..13) {
        0b000 => {
            match get_bits!(raw, 12..11) {
                0b11 => unimplemented!(),
                _ => ThumbInstruction::MSR(raw.into())
            }
        },
        0b001 => ThumbInstruction::OpImmediate(raw.into()),
        0b010 => {
            if get_bits!(raw, 12..10) == 0b001 {
                ThumbInstruction::HiOp(raw.into())
            } else {
                unimplemented!()
            }
        },
        0b011 => unimplemented!(),
        0b100 => unimplemented!(),
        0b101 => unimplemented!(),
        0b110 => unimplemented!(),
        0b111 => unimplemented!(),
        _ => unreachable!()
    }
}

