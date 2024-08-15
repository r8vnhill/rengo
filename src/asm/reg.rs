/// Represents a CPU register.
///
/// This enum is used to represent different registers within the CPU. Currently, it only includes
/// the `Rax` register, which is typically used for storing the return value of functions or
/// intermediate results during computation.
pub(crate) enum Reg {
    /// The `Rax` register.
    ///
    /// The `Rax` register is commonly used as a general-purpose register
    /// and is often the place where the result of a computation is stored.
    Rax,
}
