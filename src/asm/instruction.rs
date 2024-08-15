use crate::asm::arg::Arg;

#[derive(Debug, PartialEq, Clone)]
/// Represents an assembly instruction.
///
/// The `Instruction` enum is used to model different types of assembly instructions. It currently
/// supports three instructions: `Mov`, `Add`, and `Sub`.
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

    /// The `Add` (addition) instruction.
    ///
    /// This instruction adds the value of the second argument to the value of the first argument
    /// and stores the result in the destination argument. Both arguments can be either a register
    /// or a constant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Example of adding a constant value to the value in a register
    /// Instruction::Add(Arg::Registry(Reg::Rax), Arg::Constant(1));
    ///
    /// // Example of adding the value of one register to another
    /// Instruction::Add(Arg::Registry(Reg::Rax), Arg::Registry(Reg::Rbx));
    /// ```
    Add(Arg, Arg),

    /// The `Sub` (subtraction) instruction.
    ///
    /// This instruction subtracts the value of the second argument from the value of the first
    /// argument and stores the result in the destination argument. Both arguments can be either a
    /// register or a constant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // Example of subtracting a constant value from the value in a register
    /// Instruction::Sub(Arg::Registry(Reg::Rax), Arg::Constant(1));
    ///
    /// // Example of subtracting the value of one register from another
    /// Instruction::Sub(Arg::Registry(Reg::Rax), Arg::Registry(Reg::Rbx));
    /// ```
    Sub(Arg, Arg),
}
