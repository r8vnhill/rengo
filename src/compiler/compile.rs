use crate::asm::arg::Arg;
use crate::asm::arg::Arg::Constant;
use crate::asm::instruction::Instruction;
use crate::asm::instruction::Instruction::{Dec, Inc, Mov};
use crate::asm::reg::Reg;
use crate::ast::expr::Expression;
use Arg::{Registry, RegistryOffset};
use Expression::{Decrement, Identifier, Increment, Let, Number};
use Instruction::{Add, Sub};
use Reg::Rax;
use crate::asm::reg::Reg::Rsp;
use crate::env::{add, Env};

/// Compiles an `Expression` into a sequence of `Instruction`s.
///
/// The `compile_expression` function translates a high-level `Expression` into a low-level sequence
/// of `Instruction`s that can be executed in an assembly-like environment. The function handles
/// numeric literals, increment and decrement operations, as well as variable bindings (e.g., `let`
/// expressions) and identifiers. The generated instructions are designed to work with a simulated
/// stack and CPU registers.
///
/// # Arguments
///
/// * `expression` - A reference to an `Expression` to be compiled into assembly instructions.
/// * `env` - A mutable reference to the environment (`Env`), which maps variable names to stack
///   slots.
///
/// # Returns
///
/// * `Ok(Vec<Instruction>)` - A vector of `Instruction`s representing the compiled code if the
///   expression is successfully compiled.
/// * `Err(())` - An error indicating that the compilation failed. This can occur if an identifier
///   is not found in the environment or if another compilation step fails.
///
/// # Examples
///
/// ## Compiling a Numeric Expression
/// ```rust
/// let expr = Expression::Number(42);
/// let mut env = Env::new();
/// let instructions = compile_expression(&expr, &mut env).unwrap();
/// assert_eq!(instructions, vec![Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42))]);
/// ```
///
/// ## Compiling an Increment Expression
/// ```rust
/// let inc_expr = Expression::Increment(Box::new(Expression::Number(42)));
/// let mut env = Env::new();
/// let inc_instructions = compile_expression(&inc_expr, &mut env).unwrap();
/// assert_eq!(inc_instructions, vec![
///     Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42)),
///     Instruction::Inc(Arg::Registry(Reg::Rax))
/// ]);
/// ```
///
/// ## Compiling a Decrement Expression
/// ```rust
/// let dec_expr = Expression::Decrement(Box::new(Expression::Number(42)));
/// let mut env = Env::new();
/// let dec_instructions = compile_expression(&dec_expr, &mut env).unwrap();
/// assert_eq!(dec_instructions, vec![
///     Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42)),
///     Instruction::Dec(Arg::Registry(Reg::Rax))
/// ]);
/// ```
///
/// ## Compiling a Let Expression
/// ```rust
/// let let_expr = Expression::Let(
///     "x".to_string(),
///     Box::new(Expression::Number(42)),
///     Box::new(Expression::Identifier("x".to_string()))
/// );
/// let mut env = Env::new();
/// let let_instructions = compile_expression(&let_expr, &mut env).unwrap();
/// assert_eq!(let_instructions, vec![
///     Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42)),
///     Instruction::Mov(Arg::RegistryOffset(Reg::Rsp, -1), Arg::Registry(Reg::Rax)),
///     Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(1))
/// ]);
/// ```
///
/// # Note:
/// The `Identifier` variant expects the variable to have been previously defined in the
/// environment. If the identifier is not found, the function may return an error or panic depending
/// on the implementation.
pub(crate) fn compile_expression(expression: &Expression, env: &mut Env) -> Result<Vec<Instruction>, ()> {
    match expression {
        Expression::Number(value) => Ok(vec![Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(*value))]),
        Expression::Increment(expr) => {
            let mut instructions = compile_expression(expr, env)?;
            instructions.push(Instruction::Inc(Arg::Registry(Reg::Rax)));
            Ok(instructions)
        },
        Expression::Decrement(expr) => {
            let mut instructions = compile_expression(expr, env)?;
            instructions.push(Instruction::Dec(Arg::Registry(Reg::Rax)));
            Ok(instructions)
        },
        Expression::Let(identifier, value, body) => {
            let slot = add(identifier.clone(), env);  // Add returns the slot directly, not a new env
            let mut instructions = compile_expression(value, env)?;  // Continue using the same env
            instructions.push(Instruction::Mov(
                Arg::RegistryOffset(Reg::Rsp, -slot),  // Correctly calculate the offset
                Arg::Registry(Reg::Rax)
            ));
            let mut body_instructions = compile_expression(body, env)?;
            instructions.append(&mut body_instructions);
            Ok(instructions)
        },

        Expression::Identifier(identifier) => {
            let slot = env.get(identifier).ok_or(())?;
            Ok(vec![Mov(Registry(Rax), RegistryOffset(Rsp, -*slot))])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;
    use proptest::prelude::*;

    proptest!(
        #[test]
        fn test_compile_expression_number(value in any::<i64>()) {
            let expr = Number(value);
            let instructions = compile_expression(&expr, & mut Env::new()).unwrap();
            expect!(instructions).to(be_equal_to(vec![Mov(Registry(Rax), Constant(value))]));
        }

        #[test]
        fn test_compile_expression_increment(value in any::<i64>()) {
            let expr = Increment(Box::new(Number(value)));
            let instructions = compile_expression(&expr, & mut Env::new()).unwrap();
            expect!(instructions).to(be_equal_to(vec![
                Mov(Registry(Rax), Constant(value)),
                Inc(Registry(Rax)),
            ]));
        }

        #[test]
        fn test_compile_expression_decrement(value in any::<i64>()) {
            let expr = Decrement(Box::new(Expression::Number(value)));
            let instructions = compile_expression(&expr, & mut Env::new()).unwrap();
            expect!(instructions).to(be_equal_to(vec![
                Mov(Registry(Rax), Constant(value)),
                Dec(Registry(Rax)),
            ]));
        }

        #[test]
        fn test_compile_expression_let(value in any::<i64>()) {
            let let_expr = Let(
                "x".to_string(),
                Box::new(Number(value)),
                Box::new(Identifier("x".to_string()))
            );
            let instructions = compile_expression(&let_expr, &mut Env::new()).unwrap();
            expect!(instructions).to(be_equal_to(vec![
                Mov(Registry(Rax), Constant(value)),
                Mov(RegistryOffset(Rsp, -1), Registry(Rax)),
                Mov(Registry(Rax), RegistryOffset(Rsp, -1)), // Load the value of 'x' from the stack
            ]));
        }

        #[test]
        fn test_compile_expression_identifier(value in any::<i64>()) {
            let expr = Let(
                "x".to_string(),
                Box::new(Number(value)),
                Box::new(Identifier("x".to_string()))
            );
            let instructions = compile_expression(&expr, &mut Env::new()).unwrap();
            expect!(instructions).to(be_equal_to(vec![
                Mov(Registry(Rax), Constant(value)),                // Move the value into Rax
                Mov(RegistryOffset(Rsp, -1), Registry(Rax)),        // Store it in the stack slot for 'x'
                Mov(Registry(Rax), RegistryOffset(Rsp, -1)),        // Load the value of 'x' back into Rax
            ]));
        }
    );
    proptest!(
        #[test]
        fn test_compile_expression_identifier_not_found(value in any::<i64>()) {
            let expr = Identifier("x".to_string());
            let result = compile_expression(&expr, & mut Env::new());
            expect!(result).to(be_err());
        }
    );

    proptest!(
        #[test]
        fn test_compile_expression_mixed(value in any::<i64>()) {
            let expr = Let(
                "x".to_string(),
                Box::new(Increment(Box::new(Number(value)))),
                Box::new(Decrement(Box::new(Identifier("x".to_string()))))
            );
            let instructions = compile_expression(&expr, &mut Env::new()).unwrap();
            expect!(instructions).to(be_equal_to(vec![
                Mov(Registry(Rax), Constant(value)),
                Inc(Registry(Rax)),
                Mov(RegistryOffset(Rsp, -1), Registry(Rax)),
                Mov(Registry(Rax), RegistryOffset(Rsp, -1)),
                Dec(Registry(Rax)),
            ]));
        }
    );
}
