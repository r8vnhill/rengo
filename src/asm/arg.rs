use crate::asm::reg::Reg;

/// Represents an argument in an assembly instruction.
///
/// The `Arg` enum is used to model the operands that can be utilized in assembly instructions.
/// These operands can either be numeric constants, CPU registers, or registers with an offset,
/// providing the flexibility needed to represent different types of assembly arguments.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Arg {
    /// A numeric constant.
    ///
    /// The `Constant` variant holds a 64-bit integer (`i64`) that represents an immediate value
    /// used directly in an assembly instruction. This is often utilized in operations where
    /// a fixed numeric value is required, such as setting a value or performing arithmetic
    /// operations.
    ///
    /// # Example
    ///
    /// ```rust
    /// let arg = Arg::Constant(42);
    /// ```
    Constant(i64),

    /// A CPU register.
    ///
    /// The `Registry` variant holds a value of the `Reg` enum, representing a specific CPU
    /// register. Registers are essential for storing intermediate values, handling computation
    /// results, and managing various states within the CPU during the execution of assembly
    /// instructions.
    ///
    /// # Example
    ///
    /// ```rust
    /// let arg = Arg::Registry(Reg::Rax);
    /// ```
    Registry(Reg),

    /// A CPU register with an offset.
    ///
    /// The `RegistryOffset` variant combines a CPU register with a numeric offset. This is
    /// commonly used in assembly instructions to reference memory addresses relative to a
    /// register's current value. For instance, `Reg::Rsp` with an offset might be used to access
    /// stack-based variables or function parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// let arg = Arg::RegistryOffset(Reg::Rsp, 8);
    /// ```
    RegistryOffset(Reg, i64),
}
