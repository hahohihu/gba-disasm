use crate::types::Register;
use crate::{get_bits, get_bit};
use contracts::debug_requires;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoHiRegister {
    Lo(Register),
    Hi(u8)
}

impl LoHiRegister {
    fn new(hi_flag: bool, reg: u8) -> Self {
        if hi_flag {
            LoHiRegister::Hi(reg)
        } else {
            LoHiRegister::Lo(Register(reg))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum HiOpCode {
    ADD = 0b00,
    CMP = 0b01,
    MOV = 0b10,
    BX  = 0b11
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HiOp {
    op: HiOpCode,
    src: LoHiRegister,
    dest: LoHiRegister
}

impl From<u16> for HiOp {
    #[debug_requires(get_bits!(raw, 15..10) == 0b010001)]
    fn from(raw: u16) -> Self {
        HiOp {
            op: FromPrimitive::from_u16(get_bits!(raw, 9..8)).unwrap(),
            src: LoHiRegister::new(get_bit(raw, 6) == 1, get_bits!(raw, 5..3) as u8),
            dest: LoHiRegister::new(get_bit(raw,7) == 1, get_bits!(raw, 2..0) as u8),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::types::Register;
    use super::{HiOp, HiOpCode, LoHiRegister};
    use test_case::test_case;

    #[test_case(HiOpCode::ADD, 0b010001_00_00000000)]
    #[test_case(HiOpCode::CMP, 0b010001_01_00000000)]
    #[test_case(HiOpCode::MOV, 0b010001_10_00000000)]
    #[test_case(HiOpCode::BX, 0b010001_11_00000000)]
    fn opcode_matches(hi_op: HiOpCode, raw: u16) {
        assert_eq!(HiOp::from(raw).op, hi_op);
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn source(
            hi in any::<bool>(),
            val in 0..(1_u8 << 3)
        ) {
            let reg = if hi {
                LoHiRegister::Hi(val)
            } else {
                LoHiRegister::Lo(Register(val))
            };
            let high_reg = if hi { 1 } else { 0 } as u16;
            let hiop = HiOp::from((0b010001 << 10) | (high_reg << 6) | ((val as u16) << 3));
            prop_assert_eq!(hiop.src, reg);
        }

        #[test]
        fn dest(
            hi in any::<bool>(),
            val in 0..(1_u8 << 3)
        ) {
            let reg = if hi {
                LoHiRegister::Hi(val)
            } else {
                LoHiRegister::Lo(Register(val))
            };
            let high_reg = if hi { 1 } else { 0 } as u16;
            let hiop = HiOp::from((0b010001 << 10) | (high_reg << 7) | (val as u16));
            prop_assert_eq!(hiop.dest, reg)
        }
    }
}