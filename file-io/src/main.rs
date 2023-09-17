use std::fs;
use std::io::{self, Write};

fn main() {
    loop {
        println!("1. Add note");
        println!("2. View notes");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_note(),
            "2" => view_notes(),
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn add_note() {
    println!("Enter your note: ");

    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    writeln!(file, "{}", note).unwrap();
}

fn view_notes() {
    let notes = fs::read_to_string("notes.txt").unwrap();
    println!("{}", notes);
}
