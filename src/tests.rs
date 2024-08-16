use crate::ast::expression::Expression;
use crate::create_output_paths;

fn parse_compile_and_assemble(program: String) -> Result<(), Box<dyn std::error::Error>> {
    let ast: Expression<&str> = crate::parser::parse::parse(&crate::parser::tokenize::tokenize(&program)?)?;
    let assembly = crate::compiler::compile::compile_expression(&ast, &mut Default::default());
    let asm_output_path = create_output_paths("build/test/", "out.asm")?;
    let obj_output_path = create_output_paths("build/test/", "out.obj")?;
    let exe_output_path = create_output_paths("build/test/", "out.exe")?;
    let prelude = "section .text\n\
                   global _start\n\
                   _start:\n";
    let asm = format!("{}\n{}", prelude, crate::asm::to_string::asm_to_string(assembly.unwrap()));
    let ret = "ret\n";
    let compiled_asm = format!("{}\n{}", asm, ret);
    crate::write_assembly(&asm_output_path, &compiled_asm)?;
    crate::assemble::assemble(&asm_output_path, &obj_output_path)?;
    crate::link(&obj_output_path, &exe_output_path)?;
    Ok(())
}

fn execute(program: String) -> Result<i64, Box<dyn std::error::Error>> {
    let exe_output_path = create_output_paths("build/test/", "out.exe")?;
    let output = std::process::Command::new(exe_output_path).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let exit_code = output.status.code().unwrap();
    if exit_code != 0 {
        return Err(format!("Program exited with code {}", exit_code).into());
    }
    let result = stdout.trim().parse::<i64>()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use expectest::prelude::*;
    use proptest::prelude::*;
    use super::*;

    #[test]
    fn executes_correctly() {
        let program = "420".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(420));

        let program = "0".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(0));

        let program = "-420".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(-420));

        let program = "420--".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(419));

        let program = "420   --".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(419));

        let program = "420++".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(421));

        let program = "420   ++".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(421));

        let program = "420++--".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(420));

        let program = "420--++".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(420));

        let program = "420++--++".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(421));

        let program = "let x = 420; x".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(420));

        let program = "let x = 420; x++".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(421));

        let program = "let x = 420; let y = x++; y".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(421));

        let program = "let x = 420; let x = 69; x".to_string();
        parse_compile_and_assemble(program.clone()).unwrap();
        let result = execute(program).unwrap();
        expect!(result).to(be_equal_to(69));
    }
}