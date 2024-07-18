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
    let mut steps: Vec<f32> = Vec::new();
    steps.push(args.weight);
    steps.extend(args.buildup);
    steps.sort_by(|l, r| l.partial_cmp(r).unwrap());
    for step in steps {
        let reps = (lift / step).round() as usize;
        println!("{:>2} * {:>4.2}", reps, step);
    }
}
