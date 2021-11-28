use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use crate::types::Register;
use crate::{get_bits, get_bit};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
enum Source {
    PC = 0,
    SP = 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadAddress { 
    src: Source,
    dest: Register,
    word8: u8
}

impl From<u16> for LoadAddress { 
    fn from(raw: u16) -> Self { 
        assert!(get_bits!(raw, 15..12) == 0b1010);

        Self { 
            src: FromPrimitive::from_u8(get_bit(raw, 11)).unwrap(),
            dest: Register(get_bits!(raw, 10..8) as u8),
            word8: get_bits!(raw, 7..0) as u8
        }
    }
}

#[cfg(test)]
mod test {
    use crate::thumb::load_addr::{LoadAddress, Source};
    use crate::types::Register;
    use num_traits::FromPrimitive;
    use test_case::test_case;

    #[test_case(0b1010_0_00000000000, Source::PC ; "PC")]
    #[test_case(0b1010_1_00000000000, Source::SP ; "SP")]
    fn src(input: u16, expected_code: Source) { 
        assert_eq!(LoadAddress::from(input).src, expected_code);
    }


    use proptest::prelude::*;

    proptest! { 
        #[test]
        fn props(
            src in 0..1 as u16,
            dest in 0..0b111 as u16,
            wrd8 in 0..0b11111111 as u16
        ) {
            let loadaddr = LoadAddress {
                src: FromPrimitive::from_u8(src as u8).unwrap(),
                dest: Register(dest as u8),
                word8: wrd8 as u8
            };


            let base = 0b1010_0000_0000_0000;
            let decoded_instruction = LoadAddress::from(base | (src << 11) | (dest << 8) | wrd8);

            prop_assert_eq!(loadaddr, decoded_instruction);
        }
    }
}