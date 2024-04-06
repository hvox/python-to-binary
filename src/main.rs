use std::os::unix::fs::PermissionsExt;
use std::{env, fs};

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		eprintln!("Usage: {} source.py output.bin", args[0]);
		std::process::exit(1);
	}
	let source = match std::fs::read_to_string(&args[1]) {
		Ok(string) => string,
		_ => {
			eprintln!("ERROR: Failed to read {}", args[1]);
			std::process::exit(1);
		}
	};
	let mut output = include_bytes!("template").to_vec();
	output.extend(source.as_bytes());
	output.extend((source.len() as u32).to_le_bytes());
	std::fs::write(args[2].clone(), output).unwrap_or_else(|_| {
		eprintln!("ERROR: Failed to write {}", args[2]);
		std::process::exit(1);
	});
	let _ = fs::set_permissions(args[2].clone(), fs::Permissions::from_mode(0o777));
}
