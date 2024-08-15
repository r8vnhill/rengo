use Arg::Registry;
use Reg::Rax;
use crate::asm::arg::Arg;
use crate::asm::arg::Arg::Constant;
use crate::asm::instruction::Instruction;
use crate::asm::instruction::Instruction::Mov;
use crate::asm::reg::Reg;
use crate::ast::expr::Expression;

/// Compiles an `Expression` into a list of `Instruction`s.
///
/// This function takes an `Expression` and converts it into a corresponding sequence of assembly
/// instructions. Currently, it supports only `Number` expressions, which are compiled into a `Mov`
/// instruction that moves the numeric value into the `Rax` register.
///
/// # Arguments
///
/// * `expression` - A reference to the `Expression` to be compiled.
///
/// # Returns
///
/// * `Ok(Vec<Instruction>)` - A vector of `Instruction`s representing the compiled code if the
///     expression is successfully compiled.
/// * `Err(())` - An error indicating that the compilation failed. (Currently, this function always
///     succeeds as it only handles `Number` expressions.)
///
/// # Example
///
/// ```rust
/// let expr = Expression::Number(42);
/// let instructions = compile_expression(&expr).unwrap();
/// assert_eq!(instructions, vec![Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42))]);
/// ```
fn compile_expression(expression: &Expression) -> Result<Vec<Instruction>, ()> {
    match expression {
        Expression::Number(value) => Ok(vec![Mov(Registry(Rax), Constant(*value))]),
    }
}
