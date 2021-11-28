use crate::get_bits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnconditionalBranch { 
    offset11: u16,
}

impl From<u16> for UnconditionalBranch { 
    fn from(raw: u16) -> Self { 
        debug_assert!(get_bits!(raw, 15..11) == 0b11100);

        Self { 
            offset11: get_bits!(raw, 10..0)
        }
    }
}


#[cfg(test)]
mod test { 
    use crate::thumb::ucb::UnconditionalBranch;
    use proptest::prelude::*;

    proptest! { 
        #[test]
        fn props(offset11 in 0_u16..0b11111111111) {
            let ucb = UnconditionalBranch { 
                offset11: offset11
            };

            let base: u16 = 0b11100_00000000000;
            let decoded_instruction = UnconditionalBranch::from(base | offset11);

            prop_assert_eq!(ucb, decoded_instruction);
        }
    }
}