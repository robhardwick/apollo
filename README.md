# Apollo

[![Build Status](https://travis-ci.org/robhardwick/apollo.svg?branch=master)](https://travis-ci.org/robhardwick/apollo) ![License](https://img.shields.io/github/license/robhardwick/apollo)

A generative musical composition and synthesis system written in Rust.

For more information see the [apollo crate documentation](https://robhardwick.github.io/apollo/docs/apollo/index.html).

## CLI
```
USAGE:
    apollo-cli [FLAGS] [OPTIONS]

FLAGS:
    -h, --help           Prints help information
    -r, --seed-random    Use a random seed for random number generator
    -t, --text           Output audio samples as textual data
    -V, --version        Prints version information

OPTIONS:
    -c, --config <FILE>          Set a custom configuration file
    -l, --length <SECONDS>       Set the output length in seconds (ignored when playing audio)
    -p, --preset <ID>            Set preset to use from configuration file
    -o, --record <FILENAME>      Output audio data to specified file
    -z, --sample-rate <VALUE>    Set a custom sample rate (ignored when playing audio)
    -s, --seed <VALUE>           Set a custom seed for random number generator
```
