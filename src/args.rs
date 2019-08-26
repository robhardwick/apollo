use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::{
    Arg,
    App,
    value_t,
};
use failure::Error;
use rand::{
    thread_rng,
    Rng,
};

use apollo::Config;

const DEFAULT_CONFIG: &str = "config.json";
const DEFAULT_PRESET: &str = "default";
const DEFAULT_SAMPLE_RATE: f32 = 44100.;
const DEFAULT_LENGTH: usize = 10;

pub struct Args {
    pub config: Config,
    pub preset: String,
    pub seed: u64,
    pub sample_rate: f32,
    pub length: usize,
    pub output: ArgsOutput,
}

pub enum ArgsOutput {
    Play,
    Record(String),
    Text,
}

impl Args {
    pub fn new() -> Result<Self, Error> {
        let matches = App::new("Apollo")
            .version("0.1")
            .author("Rob Hardwick <robhardwick@gmail.com>")
            .about("Algorithmically generated music")
            .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Set a custom configuration file")
                .takes_value(true))
            .arg(Arg::with_name("preset")
                .short("p")
                .long("preset")
                .value_name("ID")
                .help("Set preset to use from configuration file")
                .takes_value(true))
            .arg(Arg::with_name("seed")
                .short("s")
                .long("seed")
                .value_name("VALUE")
                .help("Set a custom seed for random number generator")
                .takes_value(true)
                .conflicts_with("seed-random"))
            .arg(Arg::with_name("seed-random")
                .short("r")
                .long("seed-random")
                .help("Use a random seed for random number generator"))
            .arg(Arg::with_name("sample-rate")
                .short("z")
                .long("sample-rate")
                .value_name("VALUE")
                .help("Set a custom sample rate (ignored when playing audio)")
                .takes_value(true))
            .arg(Arg::with_name("length")
                .short("l")
                .long("length")
                .value_name("SECONDS")
                .help("Set the output length in seconds (ignored when playing audio)")
                .takes_value(true))
            .arg(Arg::with_name("text")
                .short("t")
                .long("text")
                .help("Output audio samples as textual data"))
            .arg(Arg::with_name("record")
                .short("o")
                .long("record")
                .value_name("FILENAME")
                .help("Output audio data to specified file")
                .takes_value(true)
                .conflicts_with("text"))
            .get_matches();

        let config_path = value_t!(matches, "config", String).unwrap_or(DEFAULT_CONFIG.to_string());
        let config_file = File::open(Path::new(&config_path))?;
        let config_reader = BufReader::new(config_file);
        let config: Config = serde_json::from_reader(config_reader)?;

        let preset = value_t!(matches, "preset", String).unwrap_or(DEFAULT_PRESET.to_string());

        let seed = if matches.is_present("seed-random") {
            let mut rng = thread_rng();
            rng.gen()
        } else {
            value_t!(matches, "seed", u64).unwrap_or(config.seed)
        };

        let sample_rate = value_t!(matches, "sample-rate", f32).unwrap_or(DEFAULT_SAMPLE_RATE);
        let length = value_t!(matches, "length", usize).unwrap_or(DEFAULT_LENGTH) * sample_rate as usize;

        let output = if matches.is_present("text") {
            ArgsOutput::Text
        } else if matches.is_present("record") {
            ArgsOutput::Record(value_t!(matches, "record", String).unwrap())
        } else {
            ArgsOutput::Play
        };

        Ok(Args {
            config,
            preset,
            seed,
            sample_rate,
            length,
            output,
        })
    }
}