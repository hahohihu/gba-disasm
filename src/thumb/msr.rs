use crate::types::{Register, Immediate};
use crate::get_bits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveShiftedRegisterOpCode {
    LSL,
    LSR,
    ASR
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveShiftedRegister {
    op: MoveShiftedRegisterOpCode, // TODO: enum
    offset: Immediate,
    src: Register,
    dest: Register
}

impl MoveShiftedRegister {
    fn parse_opcode(raw: u16) -> MoveShiftedRegisterOpCode {
        match get_bits!(raw, 12..11) {
            0 => MoveShiftedRegisterOpCode::LSL,
            1 => MoveShiftedRegisterOpCode::LSR,
            2 => MoveShiftedRegisterOpCode::ASR,
            _ => unreachable!()
        }
    }
}

impl From<u16> for MoveShiftedRegister {
    fn from(raw: u16) -> Self {
        assert!(get_bits!(raw, 15..13) == 0);
        MoveShiftedRegister {
            op: MoveShiftedRegister::parse_opcode(raw),
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

    #[test]
    fn values() {
        let matches = [(MoveShiftedRegister {
            op: MoveShiftedRegisterOpCode::LSL,
            offset: 0b11001,
            src: Register(0b100),
            dest: Register(0b011)
        }, 0b000_00_11001_100_011 as u16),
        (MoveShiftedRegister {
            op: MoveShiftedRegisterOpCode::LSR,
            offset: 0b10000,
            src: Register(0b001),
            dest: Register(0b111)
        }, 0b000_01_10000_001_111 as u16),
        (MoveShiftedRegister {
            op: MoveShiftedRegisterOpCode::ASR,
            offset: 0b11001,
            src: Register(0b100),
            dest: Register(0b011)
        }, 0b000_10_11001_100_011 as u16)];
        for (msr, binary) in matches {
            assert_eq!(ThumbInstruction::MSR(msr), decode_thumb(binary));
        }
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn values_match(
            offset in 0..0b100000 as u16,
            src in 0..0b1000 as u16,
            dest in 0..0b1000 as u16
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
