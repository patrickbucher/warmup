use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
/// Calculates the repetitions required for a warmup.
struct Args {
    /// Target weight
    #[arg(short, long)]
    weight: f32,

    /// Target repetitions
    #[arg(short, long)]
    reps: usize,

    /// Buildup weights for warmup
    #[arg(short, long)]
    buildup: Vec<f32>,
}

fn main() {
    let args = Args::parse();
    let lift: f32 = args.weight * args.reps as f32;
    for step in args.buildup {
        let reps = (lift / step).round() as usize;
        println!("{:>2} * {:>4.2}", reps, step);
    }
    println!("{:>2} * {:>4.2}", args.reps, args.weight);
}
