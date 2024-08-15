use crate::asm::arg::Arg;

/// Represents an assembly instruction.
///
/// The `Instruction` enum models various types of assembly instructions that can be used in
/// low-level programming. These instructions operate on `Arg` values, which can represent either
/// CPU registers or numeric constants.
///
/// ## Variants:
/// - `Inc(Arg)`: Increments the value in the specified argument by one.
/// - `Dec(Arg)`: Decrements the value in the specified argument by one.
/// - `Mov(Arg, Arg)`: Copies data from a source to a destination.
/// - `Add(Arg, Arg)`: Adds two values and stores the result in the destination.
/// - `Sub(Arg, Arg)`: Subtracts one value from another and stores the result in the destination.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Instruction {
    /// The `Inc` (increment) instruction.
    ///
    /// This instruction increases the value stored in the specified argument by one. The argument
    /// can be either a register or a memory location.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Example of incrementing the value in a register
    /// Instruction::Inc(Arg::Registry(Reg::Rax));
    /// ```
    Inc(Arg),

    /// The `Dec` (decrement) instruction.
    ///
    /// This instruction decreases the value stored in the specified argument by one. The argument
    /// can be either a register or a memory location.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Example of decrementing the value in a register
    /// Instruction::Dec(Arg::Registry(Reg::Rax));
    /// ```
    Dec(Arg),

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
