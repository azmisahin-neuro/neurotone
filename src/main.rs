use clap::Parser;
use hound;
use std::f32::consts::PI;

#[derive(Parser)]
struct Args {
    /// alpha, theta, delta, beta
    #[arg(short, long, default_value = "alpha")]
    mode: String,

    /// duration in seconds
    #[arg(short, long, default_value_t = 300)]
    duration: u32,

    /// output file
    #[arg(short, long, default_value = "output.wav")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let brain_freq = match args.mode.as_str() {
        "delta" => 2.0,
        "theta" => 6.0,
        "alpha" => 10.0,
        "beta" => 18.0,
        _ => 10.0,
    };

    let sample_rate = 44_100;
    let carrier_freq = 200.0;
    let amplitude = 0.8;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&args.output, spec).unwrap();

    let total_samples = sample_rate * args.duration;

    for n in 0..total_samples {
        let t = n as f32 / sample_rate as f32;

        // Isochronic pulse
        let pulse = ((t * brain_freq) % 1.0 < 0.5) as i32 as f32;

        let sample = amplitude
            * pulse
            * (2.0 * PI * carrier_freq * t).sin();

        let out = (sample * i16::MAX as f32) as i16;
        writer.write_sample(out).unwrap();
    }

    writer.finalize().unwrap();
    println!("Generated {} mode → {}", args.mode, args.output);
}
