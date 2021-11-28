#![allow(clippy::upper_case_acronyms)]
mod types;
mod thumb;

use thumb::addsub::AddSubtract;
use thumb::alu::AluOperations;
use thumb::load_addr::LoadAddress;
use thumb::load_store_ext::LoadStoreSignExtended;
use thumb::load_store_halfword::LoadStoreHalfword;
use thumb::load_store_reg::LoadStoreReg;
use thumb::msr::MoveShiftedRegister;
use thumb::op_immediate::OpImmediate;
use thumb::hi_op::HiOp;
use thumb::pcrl::PcRelativeLoad;
use thumb::ppreg::PushPopRegisters;
use thumb::conditional_branch::ConditionalBranch;
use thumb::unconditional_branch::UnconditionalBranch;

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

pub fn test_bit(input: u16, n: u8) -> bool {
    get_bit(input, n) == 1
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
    MoveShiftedRegister(MoveShiftedRegister),
    AddSubtract(AddSubtract),
    //Move/compare/add/sub immediate
    AluOperations(AluOperations),
    OpImmediate(OpImmediate),
    HiOp(HiOp),
    PcRelativeLoad(PcRelativeLoad),
    LoadStoreReg(LoadStoreReg),
    LoadStoreSignExtended(LoadStoreSignExtended),
    LoadStoreHalfword(LoadStoreHalfword),
    LoadAddress(LoadAddress),
    PushPopRegisters(PushPopRegisters),
    ConditionalBranch(ConditionalBranch),
    UnconditionalBranch(UnconditionalBranch)
}

pub fn decode_thumb(raw: u16) -> ThumbInstruction {
    match get_bits!(raw, ..13) {
        0b000 => {
            if get_bits!(raw, 12..11) != 0b11_u16 { 
                ThumbInstruction::MoveShiftedRegister(raw.into())
            } else { 
                ThumbInstruction::AddSubtract(raw.into())
            }
        },
        0b001 => ThumbInstruction::OpImmediate(raw.into()),
        0b010 => {
            match get_bits!(raw, 12..10) {
                0b000 =>  return ThumbInstruction::AluOperations(raw.into()),
                0b001 =>  return ThumbInstruction::HiOp(raw.into()),
                _     => { }
            }

            if get_bits!(raw, 12..11) == 0b01 { 
                return ThumbInstruction::PcRelativeLoad(raw.into())
            }

            let bit_12 = test_bit(raw, 12);
            let bit_9 = test_bit(raw, 9);

            if bit_12 && !bit_9 { 
                return ThumbInstruction::LoadStoreReg(raw.into())
            } else if bit_12 && bit_9 {
                return ThumbInstruction::LoadStoreSignExtended(raw.into())
            }

            unreachable!("no match found in 0b010_thumb");
        },
        0b011 => unimplemented!(),
        0b100 => {
            if !test_bit(raw, 12) { 
                return ThumbInstruction::LoadStoreHalfword(raw.into());
            } else {
                unimplemented!("SP-relative load/store")
            }
        },
        0b101 => {
            if !test_bit(raw, 12) {
                return ThumbInstruction::LoadAddress(raw.into());
            }

            if !test_bit(raw, 10) {
                unimplemented!("Add offset to stack pointer")
            } else {
                return ThumbInstruction::PushPopRegisters(raw.into());
            }
        },
        0b110 => {
            if !test_bit(raw, 10) {
                unimplemented!("Multiple load/store")
            } else { 
                if get_bits!(raw, 11..8) != 0b1111 { 
                    ThumbInstruction::ConditionalBranch(raw.into())
                } else {
                    unimplemented!("Software interrupt")
                }
            }
        },
        0b111 => {
            if !test_bit(raw, 12) {
                return ThumbInstruction::UnconditionalBranch(raw.into())
            } else {
                unimplemented!("Long branch with link")
            }
        },
        _ => unreachable!()
    }
}

