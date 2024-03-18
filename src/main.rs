mod ines_parser;
mod instruction;
mod memory;
mod p6502;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }

    if args.len() != 2 {
        println!("Invalid number of arguments.");
        return;
    }

    let _ines_content = ines_parser::parse(&args[1]);
}
