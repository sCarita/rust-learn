/*
    -------------------------
    OLD WAY [without $ cargo]
    compile: rustc hello.rs
    run: ./hello
    -------------------------
*/
mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_refs;
mod structs;
mod enums;
mod cli;
mod hashmaps;

fn main() {
    println!("🖨 print");
    print::run();

    println!("📦 vars");
    vars::run();

    println!("🌈 types");
    types::run();

    println!("💬 strings");
    strings::run();

    println!("✌️ tuples");
    tuples::run();

    println!("🎚 arrays");
    arrays::run();

    println!("🎛 vectors");
    vectors::run();

    println!("⏰ conditionals");
    conditionals::run();

    println!("⛓ loops");
    loops::run();

    println!("📡 functions");
    functions::run();

    println!("📧 pointers / references");
    pointer_refs::run();

    println!("💼 structs");
    structs::run();

    println!("🧮 enums");
    enums::run();

    println!("📺 cli");
    cli::run();

    println!("🗺 hashmaps");
    hashmaps::run();
}
