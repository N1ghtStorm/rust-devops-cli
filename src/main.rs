use clap::Parser;
use duct::cmd;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    mode: String,
}

fn main() {
    let args = Args::parse();

    let branch = match &args.mode[..]  {
        "bond" => "master",
        "carbon" => "add_carbon_credits",
        _ => todo!()
    };

    match cmd!("echo", "git", "checkout", branch).run() {
        Err(_) => println!("Error"),
        Ok(_) => {}
    }

    match cmd!("echo", "make", "run").run() {
        Err(_) => println!("Error"),
        Ok(_) => {}
    }
}
