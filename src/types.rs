use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterOrImmediate {
    Register(Register),
    Immediate(u8)
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
pub enum LoadStore { 
    Store = 0b0,
    Load = 0b1,
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
pub enum ByteWord { 
    Word = 0b0,
    Byte = 0b1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Condition {
    BEQ = 0b0000,
    BNE = 0b0001,
    BCS = 0b0010,
    BCC = 0b0011,
    BMI = 0b0100,
    BPL = 0b0101,
    BVS = 0b0110,
    BVC = 0b0111,
    BHI = 0b1000,
    BLS = 0b1001,
    BGE = 0b1010,
    BLT = 0b1011,
    BGT = 0b1100,
    BLE = 0b1101
}