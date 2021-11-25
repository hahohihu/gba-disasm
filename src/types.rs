#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register(pub u8);
pub type Immediate = u8;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterOrImmediate {
    Register(Register),
    Immediate(Immediate)
}
