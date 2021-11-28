use num_traits::FromPrimitive;

use crate::types::Condition;
use crate::get_bits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ConditionalBranch { 
    cond: Condition,
    soffset8: i8 
}

impl From<u16> for ConditionalBranch {
    fn from(raw: u16) -> Self {
        debug_assert!(get_bits!(raw, 15..12) == 0b1101);

        Self { 
            cond: FromPrimitive::from_u8(get_bits!(raw, 11..8) as u8).unwrap(),
            soffset8: get_bits!(raw, 7..0) as i8
        }
    }
}

#[cfg(test)]
mod test { 
    use crate::{thumb::cb::ConditionalBranch, types::Condition};
    use test_case::test_case;

    #[test_case(0b1101_0000_00000000, Condition::BEQ ; "BEQ")]
    #[test_case(0b1101_0001_00000000, Condition::BNE ; "BNE")]
    #[test_case(0b1101_0010_00000000, Condition::BCS ; "BCS")]
    #[test_case(0b1101_0011_00000000, Condition::BCC ; "BCC")]
    #[test_case(0b1101_0100_00000000, Condition::BMI ; "BMI")]
    #[test_case(0b1101_0101_00000000, Condition::BPL ; "BPL")]
    #[test_case(0b1101_0110_00000000, Condition::BVS ; "BVS")]
    #[test_case(0b1101_0111_00000000, Condition::BVC ; "BVC")]
    #[test_case(0b1101_1000_00000000, Condition::BHI ; "BHI")]
    #[test_case(0b1101_1001_00000000, Condition::BLS ; "BLS")]
    #[test_case(0b1101_1010_00000000, Condition::BGE ; "BGE")]
    #[test_case(0b1101_1011_00000000, Condition::BLT ; "BLT")]
    #[test_case(0b1101_1100_00000000, Condition::BGT ; "BGT")]
    #[test_case(0b1101_1101_00000000, Condition::BLE ; "BLE")]

    fn cond(input: u16, expected: Condition) {
        assert_eq!(expected, ConditionalBranch::from(input).cond);
    }

    use proptest::prelude::*;

    proptest! { 
        #[test] 
        fn props (
           soffset8 in 0..i8::MAX 
        ) {
            let cb = ConditionalBranch {
                cond: Condition::BEQ,
                soffset8: soffset8
            };

            let base: u16 = 0b1101_0000_0000_0000;
            let decoded_instruction = ConditionalBranch::from(base | (soffset8 as u16));

            prop_assert_eq!(cb, decoded_instruction);
        }
    }
}