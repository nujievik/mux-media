use super::{Config, StreamsOrder};
use crate::ffmpeg::Packet;
use std::{
    fs,
    io::{self, Write},
};

#[derive(Default)]
pub struct LogProgress {
    is_enable: bool,
    // packets/msg frequency
    progress_frequency: usize,

    first_file_size: u64,
    written_bytes: u64,
    percentage: u8,

    packet_count: usize,
}

impl LogProgress {
    pub fn new(cfg: &Config, order: &StreamsOrder) -> LogProgress {
        let is_enable = cfg.log_level.is_need_info() && cfg.jobs <= 1;

        if !is_enable {
            return LogProgress::default();
        }

        let first_file_size = match fs::metadata(order[0].src()) {
            Ok(meta) if meta.len() > 0 => meta.len(),
            _ => return LogProgress::default(),
        };

        LogProgress {
            is_enable: true,
            progress_frequency: 50,
            first_file_size,
            written_bytes: 0,
            percentage: 0,
            packet_count: 0,
        }
    }

    pub fn log(&mut self, stream_idx: usize, packet: &Packet) {
        if !self.is_enable || stream_idx != 0 {
            return;
        }

        self.written_bytes += packet.size() as u64;

        self.packet_count += 1;
        if self.packet_count < self.progress_frequency {
            return;
        }
        self.packet_count = 0;

        let p = (self.written_bytes * 100 / self.first_file_size) as u8;

        if p > self.percentage {
            self.percentage = p;
            print!("\r{:2}%", p);
            let _ = io::stdout().flush();
        } else {
            self.progress_frequency *= 2;
        }
    }
}
