use crate::asm::reg::Reg;

enum Arg {
    Constant(i64),
    Registry(Reg)
}
