/// Represents a CPU register.
///
/// The `Reg` enum is used to represent various CPU registers, which are critical components in
/// assembly language and low-level programming. Each variant corresponds to a specific register
/// that plays a distinct role in CPU operations, such as storing function return values,
/// intermediate computation results, or managing the stack pointer.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Reg {
    /// The `Rax` register.
    ///
    /// The `Rax` register is a general-purpose register commonly used in x86-64 architecture. It
    /// often holds the result of arithmetic operations or the return value of functions. `Rax` is
    /// one of the most frequently used registers in CPU operations.
    ///
    /// # Example
    ///
    /// ```rust
    /// let reg = Reg::Rax;
    /// ```
    Rax,

    /// The `Rsp` register.
    ///
    /// The `Rsp` register is the stack pointer register in x86-64 architecture. It holds the
    /// address of the top of the stack and is crucial for managing function calls, local
    /// variables, and control flow. Operations that push or pop data to/from the stack will
    /// modify the value stored in `Rsp`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let reg = Reg::Rsp;
    /// ```
    Rsp,
}
