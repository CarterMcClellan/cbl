use wasm_bindgen::prelude::*;

use crate::{interpreter::Interpreter, scanner::Scanner, parser::Parser};

#[wasm_bindgen]
pub fn execute_code(code: &str) -> Result<String, JsValue> {
    let interpreter = Interpreter::new();
    let mut scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    let expression_res = parser.parse();


    if let Ok(expression) = expression_res {
        let result = interpreter.interpret(&expression);
        match result {
            Ok(result) => return Ok(result.to_string()),
            Err(e) => eprintln!("Error: {:?}", e),
        }
    } else {
        eprintln!("Expression error: {:?}", expression_res.err());
    }

    Ok("Execution result".to_string())
}
