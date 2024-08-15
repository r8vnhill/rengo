use crate::asm::arg::Arg;
use crate::asm::instruction::Instruction;
use crate::asm::reg::Reg;

pub(crate) fn asm_to_string(instructions: Vec<Instruction>) -> String {
    instructions
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mov(dest, src) => format!("mov {}, {}", arg_to_string(dest), arg_to_string(src)),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn arg_to_string(arg: &Arg) -> String {
    match arg {
        Arg::Constant(value) => value.to_string(),
        Arg::Registry(reg) => reg_to_string(reg),
    }
}

fn reg_to_string(reg: &Reg) -> String {
    match reg {
        Reg::Rax => "rax".to_string(),
    }
}
