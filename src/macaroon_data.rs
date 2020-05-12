extern crate file;

use std::io::Result as IOResult;
use std::path::Path;

use rustc_serialize::hex::ToHex;

use bytes::Bytes;
use grpc::Metadata;
use grpc::MetadataKey;

/// Represents the bytes of the macaroon
pub struct MacaroonData {
    raw: Vec<u8>,
}

impl MacaroonData {
    /// Reads the macaroon data from a file at the path
    pub fn from_file_path<P: AsRef<Path>>(path: P) -> IOResult<Self> {
        let data = file::get(path)?;
        Ok(MacaroonData {
            raw: data.to_hex().as_bytes().to_vec(),
        })
    }

    /// Creates the `grpc::Metadata` instance that contain the provided macaroon
    pub fn metadata(&self) -> Metadata {
        let macaroon = Bytes::from(self.raw.clone());
        let mut metadata = Metadata::new();
        metadata.add(MetadataKey::from("macaroon"), macaroon);
        metadata
    }
}
