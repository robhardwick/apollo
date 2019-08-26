mod args;
mod play;
mod record;
mod text;

use failure::Error;

use args::{
    Args,
    ArgsOutput,
};

pub fn main() -> Result<(), Error> {
    let args = Args::new()?;

    match args.output {
        ArgsOutput::Play => play::run(
            args.config,
            args.preset,
            args.seed
        )?,
        ArgsOutput::Record(filename) => record::run(
            args.config,
            args.preset,
            args.seed,
            args.sample_rate,
            args.length,
            &filename
        )?,
        ArgsOutput::Text => text::run(
            args.config,
            args.preset,
            args.seed,
            args.sample_rate,
            args.length
        )?,
    };

    Ok(())
}