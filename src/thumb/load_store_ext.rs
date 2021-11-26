use crate::{get_bits, types::{Register}};
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
enum LoadStoreAction { 
    StoreHalfword = 0b00,
    LoadHalfword = 0b10,
    LoadSignExtendedByte = 0b01,
    LoadSignExtendedHalfword = 0b11
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoadStoreSignExtended {
    dest: Register,
    base: Register,
    offset: Register,
    action: LoadStoreAction
}

impl From<u16> for LoadStoreSignExtended {
    fn from(raw: u16) -> Self {
        assert!((raw & (1 << 9)) > 0);
        assert!((raw >> 12) == 0b0101);

        Self { 
            dest: Register(get_bits!(raw, 2..0) as u8),
            base: Register(get_bits!(raw, 5..3) as u8),
            offset: Register(get_bits!(raw, 8..6) as u8),
            action: FromPrimitive::from_u8(get_bits!(raw, 11..10) as u8).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    use crate::{thumb::load_store_ext::{LoadStoreAction, LoadStoreSignExtended}, types::Register};

    #[test_case(0b0101_00_1, LoadStoreAction::StoreHalfword ; "StoreHalfWord")]
    #[test_case(0b0101_10_1, LoadStoreAction::LoadHalfword ; "LoadHalfword")]
    #[test_case(0b0101_01_1, LoadStoreAction::LoadSignExtendedByte ; "LoadSignExtendedByte")]
    #[test_case(0b0101_11_1, LoadStoreAction::LoadSignExtendedHalfword ; "LoadSignExtendedHalfword")]

    fn action(input: u16, expected: LoadStoreAction) { 
        assert_eq!(expected, LoadStoreSignExtended::from(input << 9).action)
    }

    #[test_case(0b0101_00_1, 0b0   ; "000")]
    #[test_case(0b0101_00_1, 0b1   ; "001")]
    #[test_case(0b0101_00_1, 0b10  ; "010")]
    #[test_case(0b0101_00_1, 0b11  ; "011")]
    #[test_case(0b0101_00_1, 0b100 ; "100")]
    #[test_case(0b0101_00_1, 0b101 ; "101")]
    #[test_case(0b0101_00_1, 0b110 ; "110")]
    #[test_case(0b0101_00_1, 0b111 ; "111")]
    fn dest(input: u16, value: u16) { 
        assert_eq!(Register(value as u8), LoadStoreSignExtended::from((input << 9) | value).dest);
    }

    #[test_case(0b0101_00_1, 0b0   ; "000")]
    #[test_case(0b0101_00_1, 0b1   ; "001")]
    #[test_case(0b0101_00_1, 0b10  ; "010")]
    #[test_case(0b0101_00_1, 0b11  ; "011")]
    #[test_case(0b0101_00_1, 0b100 ; "100")]
    #[test_case(0b0101_00_1, 0b101 ; "101")]
    #[test_case(0b0101_00_1, 0b110 ; "110")]
    #[test_case(0b0101_00_1, 0b111 ; "111")]
    fn base(input: u16, value: u16) { 
        assert_eq!(Register(value as u8), LoadStoreSignExtended::from((input << 9) | (value << 3)).base);
    }

    #[test_case(0b0101_00_1, 0b0   ; "000")]
    #[test_case(0b0101_00_1, 0b1   ; "001")]
    #[test_case(0b0101_00_1, 0b10  ; "010")]
    #[test_case(0b0101_00_1, 0b11  ; "011")]
    #[test_case(0b0101_00_1, 0b100 ; "100")]
    #[test_case(0b0101_00_1, 0b101 ; "101")]
    #[test_case(0b0101_00_1, 0b110 ; "110")]
    #[test_case(0b0101_00_1, 0b111 ; "111")]
    fn offset(input: u16, value: u16) { 
        assert_eq!(Register(value as u8), LoadStoreSignExtended::from((input << 9) | (value << 6)).offset);
    }
}