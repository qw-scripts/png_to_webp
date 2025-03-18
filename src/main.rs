use clap::Parser;
use image::ImageReader;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use webp::Encoder;

#[derive(Parser, Debug)]
#[clap(name = "png_to_webp", version = "0.0.2", author = "qwadebot")]
struct Cli {
    #[clap(short, long)]
    input: std::path::PathBuf,

    #[clap(short, long)]
    output: std::path::PathBuf,

    #[clap(short, long, default_value_t = false)]
    verbose: bool,

    #[clap(short, long, default_value_t = 75)]
    quality: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if args.verbose {
        println!("Converting PNG to WebP with the following settings:");
        println!("Input: {:?}", args.input);
        println!("Output: {:?}", args.output);
        println!("Quality: {}", args.quality);
    }

    if !args.input.exists() {
        return Err(format!("Input file does not exist: {:?}", args.input).into());
    }

    if args.input.extension().unwrap_or_default() != "png" {
        return Err("Input file must be a PNG".into());
    }

    if let Some(parent) = args.output.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let start_time = Instant::now();

    let result = convert_png_to_webp(&args.input, &args.output, args.quality);

    match result {
        Ok(()) => {
            let duration = start_time.elapsed();
            if args.verbose {
                println!("Conversion successful!");
                println!("Time taken: {:?}", duration);

                let input_size = std::fs::metadata(&args.input)?.len();
                let output_size = std::fs::metadata(&args.output)?.len();
                let size_reduction = (1.0 - (output_size as f64 / input_size as f64)) * 100.0;

                println!("Input file size: {} bytes", input_size);
                println!("Output file size: {} bytes", output_size);
                println!("Size reduction: {:.2}%", size_reduction);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn convert_png_to_webp(
    input: &PathBuf,
    output: &PathBuf,
    quality: u8,
) -> Result<(), Box<dyn Error>> {
    let img = ImageReader::open(input)?.decode()?.to_rgba8();
    let encoder = Encoder::from_rgba(&img, img.width(), img.height());

    let quality_factor = quality as f32;
    let webp_data = encoder.encode(quality_factor);
    std::fs::write(output, &*webp_data)?;

    Ok(())
}
