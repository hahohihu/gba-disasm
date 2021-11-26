use crate::types::{Register, Immediate};
use crate::get_bits;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

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
    offset: Immediate
}

impl From<u16> for OpImmediate {
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

    #[test]
    fn values() {
        let matches = [];
        for (op_immediate, binary) in matches {
            assert_eq!(ThumbInstruction::OpImmediate(op_immediate), decode_thumb(binary));
        }
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
