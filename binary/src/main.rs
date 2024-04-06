use rustpython_vm as vm;
use std::{env, fs};
use vm::builtins::PyBaseException;

fn main() {
	rustpython::InterpreterConfig::new().init_stdlib().interpreter().enter(|vm| {
		let scope = vm.new_scope_with_builtins();
		let binary = fs::read(env::current_exe().unwrap()).unwrap();
		let source = &extract_text_from_binary(&binary);
		// let source = "\n\nprint(__file__)";
		let code_obj = vm
			.compile(source, vm::compiler::Mode::Exec, "<embedded>".to_owned())
			.map_err(|err| vm.new_syntax_error(&err, Some(source)))
			.unwrap();
		match vm.run_code_obj(code_obj, scope) {
			Err(err) => {
				println!("Error at line {:?}", err.traceback().unwrap().lineno.to_usize());
			}
			_ => {}
			// Err(PyBaseException{traceback, cause, context, suppress_context, args}) => {},
		}
	});
}

fn extract_text_from_binary(bytes: &[u8]) -> String {
	let n = bytes.len();
	let header = bytes[(n - 4)..n].try_into().unwrap_or([0, 0, 0, 0]);
	let text_len = u32::from_le_bytes(header) as usize;
	let text = String::from_utf8(bytes[(n - 4 - text_len)..(n - 4)].to_vec());
	return text.unwrap_or("".to_string());
}
