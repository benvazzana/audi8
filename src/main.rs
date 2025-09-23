use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::{Subcommand, Parser};
use hound::{Error, WavReader, WavSpec, WavWriter};

use audi8::{api::{health, transpose_wav}, transpose};

#[derive(Parser)]
#[command(name="audi8", version="1.0", about="An audio transposition tool", long_about = None)]
struct Args {
    // Allows for both CLI and API server functionality
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Cli {
        #[arg(index=1)]
        input_file: String,

        #[arg(index=2, allow_hyphen_values = true,
              value_parser = clap::value_parser!(i32).range(-12..=12))]
        num_semitones: i32,

        #[arg(short, long, default_value = "output.wav")]
        output_file: String,
    },

    Serve {
        #[arg(index=1, default_value = "127.0.0.1")]
        host: String,

        #[arg(index=2, default_value_t = 8080)]
        port: u16,
    },
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Cli { input_file, num_semitones, output_file } => {
            println!("file path: {input_file}");
            println!("pitch shift: {num_semitones}");

            let mut reader = match WavReader::open(&input_file) {
                Ok(reader) => reader,
                Err(Error::FormatError(..)) => panic!("audio must be a valid WAV file"),
                Err(err) => panic!("could not open audio file: {err}")
            };
            let spec = reader.spec();
            let mut writer = WavWriter::create(
                &output_file,
                WavSpec {
                    channels: spec.channels,
                    sample_rate: spec.sample_rate,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                },
            ).unwrap();

            transpose::transpose_audio(&mut reader, &mut writer, num_semitones);

            writer.finalize().unwrap();

            println!("successfully transposed {input_file}, saving to {output_file}");
            Ok(())
        },
        Command::Serve { host, port } => {
            env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
            HttpServer::new(|| {
                App::new()
                    .wrap(Logger::default())
                    .wrap(Cors::permissive())
                    .app_data(web::PayloadConfig::new(50 * 1024 * 1024))
                    .service(health)
                    .service(transpose_wav)
            })
            .bind((host.clone(), port))?
            .run()
            .await
        },
    }
}

