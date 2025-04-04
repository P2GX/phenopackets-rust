use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub(super) mod examples;
#[cfg(feature = "serde")]
mod serde;
mod protobuf;

fn read_path<T>(path: T) -> Vec<u8>
where
    T: AsRef<Path>,
{
    let mut buf = vec![];
    if let Ok(f) = File::open(path) {
        let mut b = BufReader::new(f);
        b.read_to_end(&mut buf)
            .expect("Reading a test file should succeed");
    };

    buf
}
