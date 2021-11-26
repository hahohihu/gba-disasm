use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use crate::types::Register;
use crate::get_bits;

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
enum AluOpCode {
    AND = 0x0,
    EOR = 0x1,
    LSL = 0x2,
    LSR = 0x3,
    ASR = 0x4,
    ADC = 0x5,
    SBC = 0x6,
    ROR = 0x7,
    TST = 0x8,
    NEG = 0x9,
    CMP = 0xA,
    CMN = 0xB,
    ORR = 0xC,
    MUL = 0xD,
    BIC = 0xE,
    MVN = 0xF
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AluOperations {
    op: AluOpCode,
    src: Register,
    dest: Register
}

impl From<u16> for AluOperations { 
    fn from(raw: u16) -> Self {
        assert!((raw >> 10) == 0b010000);
        AluOperations {
            op: FromPrimitive::from_u8(get_bits!(raw, 9..6) as u8).unwrap(),
            src: Register(get_bits!(raw, 5..3) as u8),
            dest: Register(get_bits!(raw, 2..0) as u8)
        }
    }
}


#[cfg(test)]
mod test {
    use crate::{thumb::alu::{AluOperations, AluOpCode}, types::Register};
    use test_case::test_case;

    #[test_case(0b010000_0000_000000, AluOpCode::AND ; "AND")]
    #[test_case(0b010000_0001_000000, AluOpCode::EOR ; "EOR")]
    #[test_case(0b010000_0010_000000, AluOpCode::LSL ; "LSL")]
    #[test_case(0b010000_0011_000000, AluOpCode::LSR ; "LSR")]
    #[test_case(0b010000_0100_000000, AluOpCode::ASR ; "ASR")]
    #[test_case(0b010000_0101_000000, AluOpCode::ADC ; "ADC")]
    #[test_case(0b010000_0110_000000, AluOpCode::SBC ; "SBC")]
    #[test_case(0b010000_0111_000000, AluOpCode::ROR ; "ROR")]
    #[test_case(0b010000_1000_000000, AluOpCode::TST ; "TST")]
    #[test_case(0b010000_1001_000000, AluOpCode::NEG ; "NEG")]
    #[test_case(0b010000_1010_000000, AluOpCode::CMP ; "CMP")]
    #[test_case(0b010000_1011_000000, AluOpCode::CMN ; "CMN")]
    #[test_case(0b010000_1100_000000, AluOpCode::ORR ; "ORR")]
    #[test_case(0b010000_1101_000000, AluOpCode::MUL ; "MUL")]
    #[test_case(0b010000_1110_000000, AluOpCode::BIC ; "BIC")]
    #[test_case(0b010000_1111_000000, AluOpCode::MVN ; "MVN")]

    fn alu_op(input: u16, expected_code: AluOpCode) {
        assert_eq!(AluOperations::from(input).op, expected_code);
    }

    #[test_case(0b0100000000_000_000, 0x0 ; "0")]
    #[test_case(0b0100000000_001_000, 0x1 ; "1")]
    #[test_case(0b0100000000_010_000, 0x2 ; "2")]
    #[test_case(0b0100000000_011_000, 0x3 ; "3")]
    #[test_case(0b0100000000_100_000, 0x4 ; "4")]
    #[test_case(0b0100000000_101_000, 0x5 ; "5")]
    #[test_case(0b0100000000_110_000, 0x6 ; "6")]
    #[test_case(0b0100000000_111_000, 0x7 ; "7")]

    fn rs(input: u16, expected: u8) { 
        let expected = Register(expected);
        let actual = AluOperations::from(input);

        assert_eq!(expected, actual.src);
    }


    #[test_case(0b0100000000_000_000, 0x0 ; "0")]
    #[test_case(0b0100000000_000_001, 0x1 ; "1")]
    #[test_case(0b0100000000_000_010, 0x2 ; "2")]
    #[test_case(0b0100000000_000_011, 0x3 ; "3")]
    #[test_case(0b0100000000_000_100, 0x4 ; "4")]
    #[test_case(0b0100000000_000_101, 0x5 ; "5")]
    #[test_case(0b0100000000_000_110, 0x6 ; "6")]
    #[test_case(0b0100000000_000_111, 0x7 ; "7")]

    fn rd(input: u16, expected: u8) { 
        let expected = Register(expected);
        let actual = AluOperations::from(input);

        assert_eq!(expected, actual.dest);
    }
}