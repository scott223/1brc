use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    generate: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("generate: {:?}", cli.generate);
}
