use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str;
use std::vec::Vec;
use duct::cmd;
use duct::Expression;

pub struct LSInterpreter {
	file: File,
	execute_commands: bool
}

fn environment_matches(mode: &str) -> bool {
	if out_contains(cmd!("echo", "$SHELL"), "bash") {
		return match mode { "bash" => true, _ => false };
	} else if out_contains(cmd!("wmic", "os"), "Windows") {
		return match mode { "cmd" => true, _ => false };
	} else {
		return false;
	}
}

fn out_contains(command: Expression, expected: &str) -> bool {
	return command.read().map(|o| o.contains(expected)).unwrap_or(false);
}

fn parse_command(line: String) -> Vec<String> {
	let mut result: Vec<String> = Vec::new();
	let mut in_string = false;
	let mut current = String::new();

	for c in line.chars() {
		match c {
			'"' => in_string = !in_string,
			' ' => {
				if in_string {
					current.push(c);
				} else {
					result.push(current);
					current = String::new();
				}
			},
			_ => {
				current.push(c);
			}
		}
	}
	result.push(current);

	return result;
}

fn run_line(execute_commands: &mut bool, line: String) {
	if line.starts_with("#") && line.ends_with("{") {
		let mode: &str = line
			.trim_left_matches("#")
			.split_whitespace()
			.next()
			.expect("");
		*execute_commands = environment_matches(mode);
	} else if line.starts_with("}") {
		*execute_commands = true;
	} else if *execute_commands && line.trim().len() > 0 {
		// Executes the command
		let mut parts = parse_command(line.trim().to_string()).into_iter();
		let name = parts.next().unwrap();
		let mut args: Vec<String> = Vec::new();

		while let Some(arg) = parts.next() {
			args.push(arg);
		}

		let output = cmd(name, args).read().unwrap_or_else(|_| "Error while executing: ".to_owned() + line.as_str());
		println!("{}", output);
	}
}

impl LSInterpreter {
    pub fn new(file: File) -> LSInterpreter {
		return LSInterpreter { file: file, execute_commands: true };
	}

	pub fn run(&mut self) {
		let reader = BufReader::new(&self.file);
		for line in reader.lines() {
			run_line(&mut self.execute_commands, line.unwrap());
		}
	}
}