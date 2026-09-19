use std::io::Read;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
//use std::io::Cursor;
use std::io::prelude::*;


pub trait Encoder {
    fn encode(&self, data: &mut Vec<u8>);
    fn decode(&self, data: &mut Vec<u8>);
}

pub struct GzBytes {
    log:bool
}

impl GzBytes {
    pub fn new() -> GzBytes {
        GzBytes {log:true}
    }

    // https://github.com/rust-lang/flate2-rs
    fn gzdecode(&self, bytes: &Vec<u8>) -> std::io::Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(bytes.as_slice());
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn gzencode(&self, bytes: &Vec<u8>) -> std::io::Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&bytes)?;
        let encoded = encoder.finish()?;
        Ok(encoded)
    }
}

impl Encoder for GzBytes {
    fn encode(&self, data: &mut Vec<u8>) {
        if let Ok(bytes) = self.gzencode(data) {
            *data = bytes;
        } else {
            if self.log {
                eprintln!("[gzencode_fail]");
            }
            let empty = Vec::new();
            *data = empty
        }
    }
    fn decode(&self, data: &mut Vec<u8>) {
        let decoded = self.gzdecode(data);
        if let Ok(bytes) = decoded {
            *data = bytes;
        } else {
            if self.log {
                eprintln!("[gzdecode_fail] {:?} {:?}", decoded, data);
            }
            let empty = Vec::new();
            *data = empty
        }
    }
}


