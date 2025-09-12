use clap::Parser;
use hound::{Error, WavReader};

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

    let file_path = &args.file;
    let pitch_shift = args.num_semitones;

    println!("file path: {file_path}");
    println!("pitch shift: {pitch_shift}");

    let mut reader = match WavReader::open(file_path) {
        Ok(reader) => reader,
        Err(Error::FormatError(..)) => panic!("audio must be a valid WAV file"),
        Err(err) => panic!("could not open audio file: {err}")
    };

    let num_samples = reader.samples::<i16>().count();

    println!("read {num_samples} samples");
}

