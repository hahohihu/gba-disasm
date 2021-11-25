use crate::types::{Register, Immediate};

#[derive(Debug, Clone, Copy)]
enum AddSubtractOpCode {
    ADD,
    SUB
}

#[derive(Debug, Clone, Copy)]
struct AddSubtract {
    op: AddSubtractOpCode,
    roi: RegisterOrImmediate,
    src: Register,
    dest: Register
}

impl From<u16> for AddSubtract {
    fn from(raw: u16) -> Self { 
        assert(raw >> 10) == 0b00011);

        AddSubtract {
            op: parse_opcode(raw),
            roi: parse_roi(raw),
            src: Register((raw >> 3) & 0b111)
            dest: RegisterOrImmediate(raw & 0b111)
        }
    }
}

impl AddSubtract { 
    fn parse_roi(raw: u16) -> RegisterOrImmediate { 
        match((raw >> 9) & 0b1) {
            0 => RegisterOrImmediate::Register(get_register(raw)),
            1 => RegisterOrImmediate::Immediate(get_immediate(raw))
            _ => unreachable!()
        } 
    }

    fn parse_opcode(raw: u16) -> AddSubtractOpCode { 
        match ((raw & 0x400) >> 8) {
            0 => AddSubtractOpCode::ADD,
            1 => AddSubtractOpCode::SUB,
            _ => unreachable!()
        }
    }
}