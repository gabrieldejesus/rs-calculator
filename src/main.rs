use clap::{arg, Command};

fn cli() -> Command {
	Command::new("calculator")
		.about("A simple CLI calculator in rust")
		.subcommand_required(true)
		.arg_required_else_help(true)
		.subcommand(
			Command::new("sum")
				.about("Add numbers")
				.arg(arg!(<NUMBERS> "The numbers that will be used in this operation"))
				.arg_required_else_help(true),
		)
}

fn main() {
	let matches = cli().get_matches();
	
	match matches.subcommand() {
		Some(("sum", sub_matches)) => {
			let numbers: &String = sub_matches
				.get_one::<String>("NUMBERS")
				.expect("required");
			
			let split_numbers: u32 = numbers
				.split_whitespace()
				.map(|n| n.parse::<u32>().unwrap())
				.sum();
			
			println!("Result: {}", split_numbers.to_string());
		}
		_ => unreachable!(),
	}
}
