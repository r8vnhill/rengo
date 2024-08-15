use crate::asm::arg::Arg;
use crate::asm::arg::Arg::Constant;
use crate::asm::instruction::Instruction;
use crate::asm::instruction::Instruction::Mov;
use crate::asm::reg::Reg;
use crate::ast::expr::Expression;
use Arg::Registry;
use Expression::{Decrement, Increment, Number};
use Instruction::{Add, Sub};
use Reg::Rax;

/// Compiles an `Expression` into a sequence of `Instruction`s.
///
/// The `compile_expression` function takes a reference to an `Expression` and converts it
/// into a vector of `Instruction`s, which can be executed in a simulated or actual
/// assembly environment. The function handles basic numeric values, as well as increment
/// and decrement operations, by generating the appropriate assembly instructions.
///
/// # Arguments
///
/// * `expression` - A reference to an `Expression` to be compiled into assembly instructions.
///
/// # Returns
///
/// * `Ok(Vec<Instruction>)` - A vector of `Instruction`s representing the compiled code
///   if the expression is successfully compiled.
/// * `Err(())` - An error indicating that the compilation failed. This is returned when
///   an inner compilation step fails (e.g., in a recursive call).
///
/// # Example
///
/// ```rust
/// // Example of compiling a numeric expression
/// let expr = Expression::Number(42);
/// let instructions = compile_expression(&expr).unwrap();
/// assert_eq!(instructions, vec![Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42))]);
///
/// // Example of compiling an increment expression
/// let inc_expr = Expression::Increment(Box::new(Expression::Number(42)));
/// let inc_instructions = compile_expression(&inc_expr).unwrap();
/// assert_eq!(inc_instructions, vec![
///     Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42)),
///     Instruction::Add(Arg::Registry(Reg::Rax), Arg::Constant(1))
/// ]);
///
/// // Example of compiling a decrement expression
/// let dec_expr = Expression::Decrement(Box::new(Expression::Number(42)));
/// let dec_instructions = compile_expression(&dec_expr).unwrap();
/// assert_eq!(dec_instructions, vec![
///     Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42)),
///     Instruction::Sub(Arg::Registry(Reg::Rax), Arg::Constant(1))
/// ]);
/// ```
pub(crate) fn compile_expression(expression: &Expression) -> Result<Vec<Instruction>, ()> {
    match expression {
        Number(value) => Ok(vec![Mov(Registry(Rax), Constant(*value))]),
        Increment(expr) => {
            let mut instructions = compile_expression(expr)?;
            instructions.push(Add(Registry(Rax), Constant(1)));
            Ok(instructions)
        }
        Decrement(expr) => {
            let mut instructions = compile_expression(expr)?;
            instructions.push(Sub(Registry(Rax), Constant(1)));
            Ok(instructions)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_compile_expression_number(value in any::<i64>()) {
            let expr = Number(value);
            let instructions = compile_expression(&expr).unwrap();
            expect!(instructions).to(be_equal_to(vec![Mov(Registry(Rax), Constant(value))]));
        }

        #[test]
        fn test_compile_expression_increment(value in any::<i64>()) {
            let expr = Increment(Box::new(Number(value)));
            let instructions = compile_expression(&expr).unwrap();
            expect!(instructions).to(be_equal_to(vec![
                Mov(Registry(Rax), Constant(value)),
                Add(Registry(Rax), Constant(1)),
            ]));
        }

        #[test]
        fn test_compile_expression_decrement(value in any::<i64>()) {
            let expr = Decrement(Box::new(Expression::Number(value)));
            let instructions = compile_expression(&expr).unwrap();
            expect!(instructions).to(be_equal_to(vec![
                Mov(Registry(Rax), Constant(value)),
                Sub(Registry(Rax), Constant(1)),
            ]));
        }
    }
}
