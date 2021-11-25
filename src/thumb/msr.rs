use crate::types::{Register, Immediate};

#[derive(Debug, Clone, Copy)]
enum MoveShiftedRegisterOpCode {
    LSL,
    LSR,
    ASR
}

#[derive(Debug, Clone, Copy)]
struct MoveShiftedRegister {
    op: MoveShiftedRegisterOpCode, // TODO: enum
    offset: Immediate,
    src: Register,
    dest: Register
}

impl MoveShiftedRegister {
    fn parse_opcode(raw: u16) -> MoveShiftedRegisterOpCode {
        match (raw >> 11) & 0b11 {
            0 => MoveShiftedRegisterOpCode::LSL,
            1 => MoveShiftedRegisterOpCode::LSR,
            2 => MoveShiftedRegisterOpCode::ASR,
            3 => unimplemented!("UB"),
            _ => unreachable!()
        }
    }
}

impl From<u16> for MoveShiftedRegister {
    fn from(raw: u16) -> Self {
        assert!(raw & 0xe000 == 0x000);
        MoveShiftedRegister {
            op: MoveShiftedRegister::parse_opcode(raw),
            offset: ((raw >> 6) & 0b11111) as u8,
            src: Register(((raw >> 3) & 0b111) as u8),
            dest: Register((raw & 0b111) as u8)
        }
    }
}