use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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