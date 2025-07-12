use anyhow::{Result, bail};
use flate2::read::ZlibDecoder;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionMethod {
    None = 0,
    Zlib = 1,
    Lz4 = 2,
    Zstd = 3,
}

impl CompressionMethod {
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::Zlib),
            2 => Some(Self::Lz4),
            3 => Some(Self::Zstd),
            _ => None,
        }
    }
}

pub fn decompress(data: &[u8], method: CompressionMethod, expected_size: usize) -> Result<Vec<u8>> {
    match method {
        CompressionMethod::None => {
            Ok(data.to_vec())
        },
        CompressionMethod::Zlib => {
            let mut decoder = ZlibDecoder::new(data);
            let mut result = Vec::with_capacity(expected_size);
            decoder.read_to_end(&mut result)?;
            Ok(result)
        },
        CompressionMethod::Lz4 => {
            // For LZ4, we need to handle frame format vs block format
            if data.starts_with(&[0x04, 0x22, 0x4D, 0x18]) {
                // LZ4 frame format
                let mut decoder = lz4_flex::frame::FrameDecoder::new(data);
                let mut result = Vec::new();
                decoder.read_to_end(&mut result)
                    .map_err(|e| anyhow::anyhow!("LZ4 frame decompression failed: {}", e))?;
                Ok(result)
            } else {
                // LZ4 block format - need to know the uncompressed size
                lz4_flex::decompress(data, expected_size)
                    .map_err(|e| anyhow::anyhow!("LZ4 block decompression failed: {}", e))
            }
        },
        CompressionMethod::Zstd => {
            zstd::decode_all(data)
                .map_err(|e| anyhow::anyhow!("Zstd decompression failed: {}", e))
        },
    }
}

pub fn compress(data: &[u8], method: CompressionMethod, level: i32) -> Result<Vec<u8>> {
    match method {
        CompressionMethod::None => {
            Ok(data.to_vec())
        },
        CompressionMethod::Zlib => {
            use flate2::write::ZlibEncoder;
            use flate2::Compression;
            use std::io::Write;
            
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(level as u32));
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        },
        CompressionMethod::Lz4 => {
            // Use LZ4 frame format for consistency
            let mut output = Vec::new();
            let mut encoder = lz4_flex::frame::FrameEncoder::new(&mut output);
            std::io::Write::write_all(&mut encoder, data)?;
            encoder.finish()?;
            Ok(output)
        },
        CompressionMethod::Zstd => {
            zstd::encode_all(data, level)
                .map_err(|e| anyhow::anyhow!("Zstd compression failed: {}", e))
        },
    }
}