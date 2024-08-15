use crate::asm::arg::Arg;

/// Represents an assembly instruction.
///
/// The `Instruction` enum is used to model different types of assembly instructions. Currently, it
/// only includes the `Mov` instruction, which is used to move data between registers and/or
/// constants.
pub(crate) enum Instruction {
    /// The `Mov` (move) instruction.
    ///
    /// This instruction copies data from the source argument to the destination argument. The first
    /// `Arg` represents the destination, where the value will be moved to, and the second `Arg`
    /// represents the source, from which the value will be taken.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Example of moving a constant value into a register
    /// Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42));
    ///
    /// // Example of moving the value from one register to another
    /// Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Registry(Reg::Rbx));
    /// ```
    Mov(Arg, Arg),
}
