use crate::types::Register;
use crate::get_bits;
use contracts::debug_requires;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
enum MoveShiftedRegisterOpCode {
    LSL = 0,
    LSR = 1,
    ASR = 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveShiftedRegister {
    op: MoveShiftedRegisterOpCode,
    offset: u8,
    src: Register,
    dest: Register
}

impl From<u16> for MoveShiftedRegister {
    #[debug_requires(get_bits!(raw, 15..13) == 0)]
    #[debug_requires(get_bits!(raw, 12..11) != 0b11)]
    fn from(raw: u16) -> Self {
        MoveShiftedRegister {
            op: FromPrimitive::from_u16(get_bits!(raw, 12..11)).unwrap(),
            offset: get_bits!(raw, 10..6) as u8,
            src: Register(get_bits!(raw, 5..3) as u8),
            dest: Register(get_bits!(raw, 2..0) as u8)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::thumb::msr::{MoveShiftedRegister, MoveShiftedRegisterOpCode};
    use crate::{Register, ThumbInstruction, decode_thumb};
    use test_case::test_case;

    #[test_case(MoveShiftedRegister {
        op: MoveShiftedRegisterOpCode::LSL,
        offset: 0b11001,
        src: Register(0b100),
        dest: Register(0b011)
    }, 0b000_00_11001_100_011)]
    #[test_case(MoveShiftedRegister {
        op: MoveShiftedRegisterOpCode::LSR,
        offset: 0b10000,
        src: Register(0b001),
        dest: Register(0b111)
    }, 0b000_01_10000_001_111)]
    #[test_case(MoveShiftedRegister {
        op: MoveShiftedRegisterOpCode::ASR,
        offset: 0b11001,
        src: Register(0b100),
        dest: Register(0b011)
    }, 0b000_10_11001_100_011)]
    fn opcode(msr: MoveShiftedRegister, binary: u16) {
        assert_eq!(ThumbInstruction::MSR(msr), decode_thumb(binary));
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn values_match(
            offset in 0..(1 << 5) as u16,
            src in 0..(1 << 3) as u16,
            dest in 0..(1 << 3) as u16
        ) {
            let msr = MoveShiftedRegister {
                op: MoveShiftedRegisterOpCode::LSL,
                offset: offset as u8,
                src: Register(src as u8),
                dest: Register(dest as u8)
            };
            prop_assert_eq!(ThumbInstruction::MSR(msr), decode_thumb((offset << 6) | (src << 3) | dest));
        }
    }
}
