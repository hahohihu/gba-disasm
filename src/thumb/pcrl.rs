use crate::get_bits;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcRelativeLoad { 
    immediate: u8,
    dest: u8
}

impl From<u16> for PcRelativeLoad { 
    fn from(raw: u16) -> Self {
        assert!((raw >> 11) == 0b01001);
        PcRelativeLoad { 
            immediate: get_bits!(raw, 7..0) as u8,
            dest: get_bits!(raw, 10..8) as u8
        }
    }
}

#[cfg(test)]
mod test {
    use crate::thumb::pcrl::PcRelativeLoad;
    use test_case::test_case;

    #[test]
    fn word8() { 
        assert_eq!(PcRelativeLoad::from(0b01001000_00000001).immediate, 1);
    }

    #[test_case(0b01001000_00000000, 0x0 ; "0")]
    #[test_case(0b01001001_00000000, 0x1 ; "1")]
    #[test_case(0b01001010_00000000, 0x2 ; "2")]
    #[test_case(0b01001011_00000000, 0x3 ; "3")]
    #[test_case(0b01001100_00000000, 0x4 ; "4")]
    #[test_case(0b01001101_00000000, 0x5 ; "5")]
    #[test_case(0b01001110_00000000, 0x6 ; "6")]
    #[test_case(0b01001111_00000000, 0x7 ; "7")]

    fn rd(input: u16, expected: u8) { 
        let actual = PcRelativeLoad::from(input);

        assert_eq!(expected, actual.dest);
    }
}