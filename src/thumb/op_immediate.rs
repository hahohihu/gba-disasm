use crate::types::Register;
use crate::get_bits;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use contracts::debug_requires;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum OpImmediateOpCode {
    MOV = 0,
    CMP = 1,
    ADD = 2,
    SUB = 3
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpImmediate {
    op: OpImmediateOpCode,
    dest: Register,
    offset: u8
}

impl From<u16> for OpImmediate {
    #[debug_requires(get_bits!(raw, 15..13) == 0b001)]
    fn from(raw: u16) -> Self {
        OpImmediate {
            op: FromPrimitive::from_u16(get_bits!(raw, 12..11)).unwrap(),
            dest: Register(get_bits!(raw, 10..8) as u8),
            offset: get_bits!(raw, 7..0) as u8
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{Register, ThumbInstruction, decode_thumb, thumb::op_immediate::{OpImmediate, OpImmediateOpCode}};
    use test_case::test_case;

    #[test_case(OpImmediate { 
        op: OpImmediateOpCode::MOV, 
        dest: Register(0b101), 
        offset: 0b00010001 
    }, 0b001_00_101_00010001)]
    #[test_case(OpImmediate { 
        op: OpImmediateOpCode::CMP, 
        dest: Register(0b101), 
        offset: 0b00010001 
    }, 0b001_01_101_00010001)]
    #[test_case(OpImmediate { 
        op: OpImmediateOpCode::ADD, 
        dest: Register(0b000), 
        offset: 0b00000000 
    }, 0b001_10_000_00000000)]
    #[test_case(OpImmediate { 
        op: OpImmediateOpCode::SUB, 
        dest: Register(0b001), 
        offset: 0b11111110 
    }, 0b001_11_001_11111110)]
    fn opcode(opcode: OpImmediate, binary: u16) {
        assert_eq!(ThumbInstruction::OpImmediate(opcode), decode_thumb(binary));
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn values_match(
            dest in 0..(1 << 3) as u16,
            offset in 0..(1 << 8) as u16,
        ) {
            let op_immediate = OpImmediate {
                op: OpImmediateOpCode::MOV,
                offset: offset as u8,
                dest: Register(dest as u8)
            };
            prop_assert_eq!(ThumbInstruction::OpImmediate(op_immediate), decode_thumb((1 << 13) | (dest << 8) | offset));
        }
    }
}
