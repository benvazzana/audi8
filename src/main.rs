use clap::Parser;

#[derive(Parser)]
#[command(name="audi8", version="1.0", about="A CLI audio transposition tool", long_about = None)]
struct Args {
    #[arg(index=1)]
    file: String,

    #[arg(index=2, allow_hyphen_values = true,
          value_parser = clap::value_parser!(i32).range(-12..=12))]
    num_semitones: i32
}

fn main() {
    let args = Args::parse();

    println!("file path: {}", args.file);
    println!("pitch shift: {}", args.num_semitones);
}

