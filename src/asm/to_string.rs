use crate::asm::arg::Arg;
use crate::asm::instruction::Instruction;
use crate::asm::reg::Reg;
use Arg::{Constant, Registry};
use Instruction::{Add, Mov, Sub};
use Reg::{Rax, Rsp};

/// Converts a vector of assembly `Instruction`s into a formatted assembly code string.
///
/// The `asm_to_string` function takes a list of `Instruction`s and converts them into a string
/// representation of assembly code. Each instruction is translated into its corresponding assembly
/// syntax and joined with newline characters.
///
/// # Arguments
///
/// * `instructions` - A vector of `Instruction`s that represent the assembly code.
///
/// # Returns
///
/// * A `String` containing the formatted assembly code.
///
/// # Example
///
/// ```rust
/// let instructions = vec![Instruction::Mov(Arg::Registry(Reg::Rax), Arg::Constant(42))];
/// let asm_code = asm_to_string(instructions);
/// assert_eq!(asm_code, "mov rax, 42");
/// ```
pub(crate) fn asm_to_string(instructions: Vec<Instruction>) -> String {
    instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Inc(dest) => format!("inc {}", arg_to_string(dest)),
            Instruction::Dec(dest) => format!("dec {}", arg_to_string(dest)),
            Mov(dest, src) => format!("mov {}, {}", arg_to_string(dest), arg_to_string(src)),
            Add(dest, src) => format!("add {}, {}", arg_to_string(dest), arg_to_string(src)),
            Sub(dest, src) => format!("sub {}, {}", arg_to_string(dest), arg_to_string(src)),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Converts an `Arg` to its string representation.
///
/// The `arg_to_string` function translates an `Arg` (which can be a constant value or a register)
/// into a string that represents its value in assembly code.
///
/// # Arguments
///
/// * `arg` - A reference to an `Arg` to be converted.
///
/// # Returns
///
/// * A `String` representing the argument in assembly code.
///
/// # Example
///
/// ```rust
/// let arg = Arg::Constant(42);
/// let arg_str = arg_to_string(&arg);
/// assert_eq!(arg_str, "42");
/// ```
fn arg_to_string(arg: &Arg) -> String {
    match arg {
        Constant(value) => value.to_string(),
        Registry(reg) => reg_to_string(reg),
        Arg::RegistryOffset(reg, offset) => format!("{} + {}", reg_to_string(reg), offset),
    }
}

/// Converts a `Reg` enum variant to its corresponding assembly string representation.
///
/// The `reg_to_string` function takes a reference to a `Reg` enum variant, which represents a CPU
/// register, and returns a `String` that represents the name of the register in assembly language
/// syntax. This function is useful when generating assembly code from higher-level representations.
///
/// # Arguments
///
/// * `reg` - A reference to a `Reg` enum variant that you want to convert to a string.
///
/// # Returns
///
/// * A `String` representing the name of the CPU register in assembly language.
///
/// # Example
///
/// ```rust
/// let reg = Reg::Rax;
/// let reg_str = reg_to_string(&reg);
/// assert_eq!(reg_str, "rax");
/// ```
fn reg_to_string(reg: &Reg) -> String {
    match reg {
        Rax => "rax".to_string(),
        Rsp => "rsp".to_string(),
    }
}

#[cfg(test)]
mod tests {
    // Add this line
    use super::*;
    use expectest::prelude::*;
    use proptest::prelude::*;
    use proptest::strategy::Strategy;

    mod reg_to_string {
        use super::*;

        #[test]
        fn it_converts_rax_to_string() {
            let reg = Reg::Rax;
            let reg_str = reg_to_string(&reg);
            expect!(reg_str).to(be_equal_to("rax"));
        }
    }

    mod arg_to_string {
        use super::*;

        proptest!(
            #[test]
            fn it_converts_constant_to_string(value in any::<i64>()) {
                let arg = Constant(value);
                let arg_str = arg_to_string(&arg);
                expect!(arg_str).to(be_equal_to(value.to_string()));
            }

            #[test]
            fn it_converts_registry_to_string(reg in prop_oneof![Just(Rax)]) {
                let arg = Registry(reg.clone());
                let arg_str = arg_to_string(&arg);
                expect!(arg_str).to(be_equal_to(reg_to_string(&reg)));
            }
        );
    }

    mod asm_to_string {
        use super::*;

        proptest!(
            #[test]
            fn it_converts_instructions_to_string(
                instructions in proptest::collection::vec(
                    prop_oneof![
                        any::<i64>().prop_map(|value| Mov(Registry(Rax), Constant(value))),
                        any::<i64>().prop_map(|value| Add(Registry(Rax), Constant(value))),
                        any::<i64>().prop_map(|value| Sub(Registry(Rax), Constant(value))),
                    ],
                    1..100,
                )
            ) {
                let asm_code = asm_to_string(instructions.clone());
                let expected = instructions
                    .iter()
                    .map(|instruction| match instruction {
                        Instruction::Inc(dest) => format!("inc {}", arg_to_string(dest)),
                        Instruction::Dec(dest) => format!("dec {}", arg_to_string(dest)),
                        Mov(dest, src) => format!("mov {}, {}", arg_to_string(dest), arg_to_string(src)),
                        Add(dest, src) => format!("add {}, {}", arg_to_string(dest), arg_to_string(src)),
                        Sub(dest, src) => format!("sub {}, {}", arg_to_string(dest), arg_to_string(src)),
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                expect!(asm_code).to(be_equal_to(expected));
            }
        );
    }
}
