use clap::Parser;
use duct::cmd;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    mode: String,

    #[clap(short, long, default_value = "./")]
    dir: String,
}

fn main() {
    let args = Args::parse();

    let branch = match &args.mode[..]  {
        "bond" => "master",
        "carbon" => "add_carbon_credits",
        _ => todo!()
    };

    match cmd!("echo", "git", "clone", "https://github.com/EvercityEcosystem/smart-sustainable-bond.git", args.dir.clone()).run() {
        Err(_) => println!("Error"),
        Ok(_) => {},
    }

    match cmd!("echo", "cd",  args.dir).run() {
        Err(_) => println!("Error"),
        Ok(_) => {},
    }

    match cmd!("echo", "git", "checkout", branch).run() {
        Err(_) => println!("Error"),
        Ok(_) => {},
    }

    match cmd!("echo", "make", "run").run() {
        Err(_) => println!("Error"),
        Ok(_) => {},
    }
}
