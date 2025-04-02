use clap::Parser;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::{self, Write};

fn generate_password(size: u32) -> String {

    // TODO: 
    // - mudar a str de chars para uma autogerada
    // - size minimo aceite tem de ser 12
    // - ... 

    let charkey = "";
    let todas = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_=!#$%&()*+,-.:'/?@";

    let mut rng = rand::rng();
    let mut password  = String::new();

    for _ in 0..size {
        password.push(todas.chars().choose(&mut rng).unwrap());
    }
 
    return password;
}

// TODO: 
// decidir guardar em ficheiro cifrado ou password é guardada cifrada mesmo sendo aleatoria????
fn save_pass(file: &str, name: &str, password: &str) -> io::Result<()>{
    let mut file = OpenOptions::new().create(true).append(true).open(file)?;

    writeln!(file, "{}: {}", name, password)?;

    Ok(())
}

// FIXME: 
// Organizar código!!

/// Simple program
#[derive(Parser, Debug)]
#[command(name = "Minipass")]
#[command(about = "Passwords geradas aleotoriamente e guardadas num ficheiro", long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    size: u32,

    #[arg(short, long, default_value = "passwords.txt")]
    file: String,
}


fn main() {
    //let pass = generate_password(12);
    //println!("Password: {}", pass);

    let args = Args::parse();

    let password = generate_password(args.size);
    println!("Password => {}", password);

    if let Err(e) = save_pass(&args.file, &args.name, &password) {
        eprintln!("erro ao guardar: {}", e);
    } else {
        println!("Password guardada com sucesso em {}", args.file);
    }
}


