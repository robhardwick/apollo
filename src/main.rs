mod args;
mod audio;
mod text;

use failure::Error;

use args::{
    Args,
    ArgsOutput,
};

pub fn main() -> Result<(), Error> {
    let args = Args::new()?;

    match args.output {
        ArgsOutput::Text => text::run(args.config, args.preset, args.sample_rate)?,
        ArgsOutput::Audio => audio::run(args.config, args.preset)?,
    };

    Ok(())
}