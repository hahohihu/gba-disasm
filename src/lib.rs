#[derive(Debug, Clone, Copy)]
struct Register(u8);
type Immediate = u8;
#[derive(Debug, Clone, Copy)]
enum RegisterOrImmediate {
    Register(Register),
    Immediate(Immediate)
}

#[derive(Debug, Clone, Copy)]
enum LoHiRegister {
    Lo(Register),
    Hi(u8)
}

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

#[derive(Debug, Clone, Copy)]
enum MoveCompareAddSubtractImmediateOpCode {
    MOV,
    CMP,
    ADD,
    SUB
}

#[derive(Debug, Clone, Copy)]
struct MoveCompareAddSubtractImmediate {
    op: MoveCompareAddSubtractImmediateOpCode,
    dest: Register,
    offset: Immediate
}

#[derive(Debug, Clone, Copy)]
enum AluOperationsOpCode {
    AND,
    EOR,
    LSL,
    LSR,
    ASR,
    ADC,
    SBC,
    ROR,
    TST,
    NEG,
    CMP,
    CMN,
    ORR,
    MUL,
    BIC,
    MVN
}

#[derive(Debug, Clone, Copy)]
struct AluOperations {
    op: AluOperationsOpCode,
    src: Register,
    dest: Register
}

#[derive(Debug, Clone, Copy)]
enum HiRegisterOpCode {
    ADD,
    CMP,
    MOV,
    BX
}

#[derive(Debug, Clone, Copy)]
struct HiRegisterOp {
    opcode: HiRegisterOpCode,
    src: LoHiRegister,
    dst: LoHiRegister
}