
use anyhow::ensure;
use crate::decode;
use std::io::Read;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]

#[derive(Debug)]
pub struct Torrent {
    pub announcement: String,
    pub info: TorrentInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentInfo {
    pub length: usize,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: usize,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
}



pub fn file_contents(path: &str) -> anyhow::Result<Torrent> {
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
    let info: &serde_json::Map<String, serde_json::Value> = data.get("info").expect("expected info").as_object().expect("expected object");

    Ok(Torrent {
        announcement: data.get("announce").expect("expected announce").as_str().expect("expected string").to_string(),
        info: TorrentInfo {
            length: info.get("length").expect("expected length").as_u64().expect("expected u64") as usize,
            name: info.get("name").expect("expected name").as_str().expect("expected string").to_string(),
            piece_length: info.get("piece length").expect("expected piece length").as_u64().expect("expected u64") as usize,
            pieces: info.get("pieces").expect("expected pieces").as_array().expect("expected array").iter().map(|x| x.as_u64().expect("expected u64") as u8).collect()
        }
    })
}