use crate::asm::reg::Reg;

/// Represents an argument in an assembly instruction.
///
/// The `Arg` enum is used to distinguish between different types of operands that can be used in
/// assembly instructions. It supports both numeric constants and CPU registers as valid arguments.
#[derive(Debug, PartialEq)]
pub(crate) enum Arg {
    /// A numeric constant.
    ///
    /// This variant holds an `i64` value, representing a constant number that is used directly in
    /// an instruction. Numeric constants are typically used for operations that involve immediate
    /// values.
    Constant(i64),

    /// A CPU register.
    ///
    /// This variant holds a value of the `Reg` enum, representing a specific CPU register.
    /// Registers are used to store intermediate values and results of computations in assembly
    /// instructions.
    Registry(Reg),
}
