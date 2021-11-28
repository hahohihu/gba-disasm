use crate::types::{Register, RegisterOrImmediate};
use crate::{get_bits, get_bit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AddSubtractOpCode {
    ADD,
    SUB
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AddSubtract {
    op: AddSubtractOpCode,
    roi: RegisterOrImmediate,
    src: Register,
    dest: Register
}

impl From<u16> for AddSubtract {
    fn from(raw: u16) -> Self { 
        assert!(get_bits!(raw, 15..11) == 0b00011);

        AddSubtract {
            op: AddSubtract::parse_opcode(raw),
            roi: AddSubtract::parse_roi(raw),
            src: Register(get_bits!(raw, 5..3) as u8),
            dest: Register(get_bits!(raw, 2..0) as u8)
        }
    }
}

impl AddSubtract { 
    fn parse_roi(raw: u16) -> RegisterOrImmediate { 
        match (raw >> 10) & 1 {
            0 => RegisterOrImmediate::Register(Register(get_bits!(raw, 8..6) as u8)),
            1 => RegisterOrImmediate::Immediate(get_bits!(raw, 8..6) as u8),
            _ => unreachable!()
        } 
    }

    fn parse_opcode(raw: u16) -> AddSubtractOpCode {
        let value = get_bit(raw, 9);
        match value {
            0 => AddSubtractOpCode::ADD,
            1 => AddSubtractOpCode::SUB,
            _ => unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::thumb::addsub::{AddSubtractOpCode, AddSubtract};
    use crate::types::{Register, RegisterOrImmediate};

    #[test] 
    fn op_0_add() { 
        let expected = AddSubtractOpCode::ADD;
        let actual = AddSubtract::from(0b0001100000000000);

        assert_eq!(expected, actual.op);
    }

    #[test]
    fn op_1_sub() { 
        let expected = AddSubtractOpCode::SUB;
        let actual = AddSubtract::from(0b0001_1010_0000_0000);

        assert_eq!(expected, actual.op);
    }

    #[test]
    fn i_0_register() {
        let expected = Register(0b111);
        let actual = AddSubtract::from(0b0001_1001_1100_0000).roi;

        assert_eq!(RegisterOrImmediate::Register(expected), actual)
    }

    #[test]
    fn i_1_immediate() { 
        let expected = 0b111;
        let actual = AddSubtract::from(0b0001_1101_1100_0000).roi;

        assert_eq!(RegisterOrImmediate::Immediate(expected), actual);
    }

    #[test]
    fn rs () { 
        let mut expected = Register(0b010);
        let mut actual = AddSubtract::from(0b0001_1000_0001_0000);
        
        assert_eq!(expected, actual.src);

        expected = Register(0b111);
        actual = AddSubtract::from(0b0001_1000_0011_1000);

        assert_eq!(expected, actual.src);

        expected = Register(0b001);
        actual = AddSubtract::from(0b0001_1000_0000_1000);

        assert_eq!(expected, actual.src);
    }

    #[test]
    fn rd () {
        let mut expected = Register(0b001);
        let mut actual = AddSubtract::from(0b0001_1000_0000_0001);

        assert_eq!(expected, actual.dest);

        expected = Register(0b010);
        actual = AddSubtract::from(0b0001_1000_0000_0010);

        assert_eq!(expected, actual.dest);

        expected = Register(0b100);
        actual = AddSubtract::from(0b0001_1000_0000_0100);

        assert_eq!(expected, actual.dest);
    }
}