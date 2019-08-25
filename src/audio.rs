use cpal::traits::{
    DeviceTrait,
    EventLoopTrait,
    HostTrait,
};
use failure::{
    Error,
    err_msg,
};

use apollo::{
    Apollo,
    Config,
};

pub fn run(config: Config, preset: String) -> Result<(), Error> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or(err_msg("Unable to get default output device"))?;
    let format = device.default_output_format()?;
    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format)?;
    event_loop.play_stream(stream_id.clone())?;

    let channels = format.channels as usize;
    let sample_rate = format.sample_rate.0 as f32;

    let mut apollo = Apollo::new(config, preset, sample_rate)?.into_iter();

    event_loop.run(move |id, result| {
        let data = match result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Output stream error {:?}: {}", id, err);
                return;
            }
        };

        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for (sample, value) in buffer.chunks_mut(channels).zip(apollo.by_ref()) {
                    let value = value as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for (sample, value) in buffer.chunks_mut(channels).zip(apollo.by_ref()) {
                    let value = (value * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for (sample, value) in buffer.chunks_mut(channels).zip(apollo.by_ref()) {
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }
    });
}