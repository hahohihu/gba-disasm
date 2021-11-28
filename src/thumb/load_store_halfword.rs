use num_traits::FromPrimitive;
use crate::{get_bit, get_bits, types::{Register, LoadStore}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadStoreHalfword { 
    dest: Register,
    base: Register,
    offset: u8,
    mode: LoadStore
}

impl From<u16> for LoadStoreHalfword { 
    fn from (raw: u16) -> Self { 
        assert!(get_bits!(raw, 15..12) == 0b1000);

        Self { 
            dest: Register(get_bits!(raw, 2..0) as u8),
            base: Register(get_bits!(raw, 5..3) as u8),
            offset: get_bits!(raw, 10..6) as u8,
            mode: FromPrimitive::from_u8(get_bit(raw, 11)).unwrap()
        }
    }
}

#[cfg(test)]
mod test { 
    use test_case::test_case;
    use crate::{thumb::load_store_halfword::{LoadStore, LoadStoreHalfword}, types::Register};

    #[test_case(0b1000, 0b0   ; "000")]
    #[test_case(0b1000, 0b1   ; "001")]
    #[test_case(0b1000, 0b10  ; "010")]
    #[test_case(0b1000, 0b11  ; "011")]
    #[test_case(0b1000, 0b100 ; "100")]
    #[test_case(0b1000, 0b101 ; "101")]
    #[test_case(0b1000, 0b110 ; "110")]
    #[test_case(0b1000, 0b111 ; "111")]
    fn dest(input: u16, value: u16) { 
        assert_eq!(Register(value as u8), LoadStoreHalfword::from((input << 12) | value).dest);
    }

    #[test_case(0b1000, 0b0   ; "000")]
    #[test_case(0b1000, 0b1   ; "001")]
    #[test_case(0b1000, 0b10  ; "010")]
    #[test_case(0b1000, 0b11  ; "011")]
    #[test_case(0b1000, 0b100 ; "100")]
    #[test_case(0b1000, 0b101 ; "101")]
    #[test_case(0b1000, 0b110 ; "110")]
    #[test_case(0b1000, 0b111 ; "111")]
    fn base(input: u16, value: u16) { 
        assert_eq!(Register(value as u8), LoadStoreHalfword::from((input << 12) | (value << 3)).base);
    }

    use proptest::prelude::*;
    
    proptest! { 
        #[test]
        fn offset(offset in 0..0b11111 as u16) {
            prop_assert_eq!(offset as u8, LoadStoreHalfword::from((1 << 15) | (offset << 6)).offset)
        }
    }

    #[test_case(0b1000, 0, LoadStore::Store ; "STORE")]
    #[test_case(0b1000, 1, LoadStore::Load  ;  "LOAD")]
    fn mode(template: u16, value: u16, expected: LoadStore) { 
        assert_eq!(expected, LoadStoreHalfword::from((template << 12) | (value << 11)).mode)
    }
}
