
use anyhow::ensure;
use crate::decode;
use std::io::Read;

#[allow(dead_code)]
pub struct Info {
    pub(crate) length: usize,
    pub(crate) name: String,
    pub(crate) piece_length: usize,
    pieces: String,
}
pub struct FileData {
    pub(crate) announcement: String,
    pub(crate) info: Info,
}
pub fn file_contents(path: &str) -> anyhow::Result<FileData> {
    let path = std::path::Path::new((&path).into());
    ensure!(path.is_file(), "Info only works on files");

    let mut file = std::fs::File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();
    let size = file.read_to_end(&mut buffer)?;
    ensure!(size > 0, format!("file {:?} is empty", &path));
    ensure!(buffer.len() == size, format!("buffer is not {size} long"));

    // Convert buffer to String
    let string_data = String::from_utf8_lossy(&buffer);


    let binding: decode::DecodedValue = decode(&string_data).unwrap();

    let data = binding.value.as_object().expect("expected object");
    let info = data
        .get("info")
        .expect("expected info")
        .as_object()
        .unwrap();
    let length = info
        .get("length")
        .unwrap()
        .as_u64()
        .expect("expected length") as usize;
    let name = info
        .get("name")
        .unwrap()
        .as_str()
        .expect("expected name")
        .to_owned();
    let piece_length = info
        .get("piece length")
        .expect("expected piece length")
        .as_u64()
        .expect("expected conversion to u64") as usize;
    let pieces = info
        .get("pieces")
        .expect("expected pieces")
        .as_str()
        .expect("as an str")
        .to_owned();
    let info_val = Info {
        length,
        name,
        piece_length,
        pieces,
    };
    let announce = data
        .get("announce")
        .expect("expected key 'announce'")
        .as_str()
        .expect("expected key 'announce' to as_str")
        .to_owned();
    Ok(FileData {
        announcement: announce,
        info: info_val,
    })
}