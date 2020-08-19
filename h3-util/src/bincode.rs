
use std::io;
use serde::Serialize;

pub fn serialize_into<W, T: ?Sized>(writer: W, value: &T, compress: bool) -> bincode::Result<()> where W: io::Write, T:Serialize {
    if compress {
        let encoder = flate2::write::GzEncoder::new(writer, flate2::Compression::fast());
        bincode::serialize_into(encoder, value)
    } else {
        bincode::serialize_into(writer, value)
    }
}