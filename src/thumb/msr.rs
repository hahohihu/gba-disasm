use crate::types::{Register, Immediate};
use crate::{ThumbInstruction, CPU, decode_thumb};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MoveShiftedRegisterOpCode {
    LSL,
    LSR,
    ASR
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveShiftedRegister {
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

impl ThumbInstruction for MoveShiftedRegister {
    fn execute(&self, cpu: &mut CPU) {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use crate::thumb::msr::{MoveShiftedRegister, MoveShiftedRegisterOpCode};
    use crate::Register;

    #[test]
    fn values() {
        assert_eq!(MoveShiftedRegister {
            op: MoveShiftedRegisterOpCode::LSL,
            offset: 0b11001,
            src: Register(0b100),
            dest: Register(0b011)
        }, MoveShiftedRegister::from(0b000_00_11001_100_011));
        assert_eq!(MoveShiftedRegister {
            op: MoveShiftedRegisterOpCode::LSR,
            offset: 0b10000,
            src: Register(0b001),
            dest: Register(0b111)
        }, MoveShiftedRegister::from(0b000_01_10000_001_111));
        assert_eq!(MoveShiftedRegister {
            op: MoveShiftedRegisterOpCode::ASR,
            offset: 0b11001,
            src: Register(0b100),
            dest: Register(0b011)
        }, MoveShiftedRegister::from(0b000_10_11001_100_011));
    }
}