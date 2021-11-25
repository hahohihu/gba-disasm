#[derive(Debug, Clone, Copy)]
pub struct Register(pub u8);
pub type Immediate = u8;
#[derive(Debug, Clone, Copy)]
pub enum RegisterOrImmediate {
    Register(Register),
    Immediate(Immediate)
}
