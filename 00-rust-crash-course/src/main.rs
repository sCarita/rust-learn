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
mod files;

fn modifies(x: &mut f64) {
    *x = 1.0;
}

fn main() {
    let mut res: f64 = 0.0;
    modifies(&mut res);
    println!("res is {}", res);

    let mut w: i32 = 7;
    w = 10;

    println!("{:?}", w);

    let x: &i32 = &7;
    assert_eq!(*x, 7);
    let y: &mut i32 = &mut 9;
    *y = 11;
    assert_eq!(*y, 11);

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

    println!("🗄 files");
    files::run();
}
