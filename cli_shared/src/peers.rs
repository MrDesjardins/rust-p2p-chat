use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use crate::{
    default_values::{DEFAULT_IP, DEFAULT_PORT, PEERS_FILE},
    models::PeerInfo,
};

pub fn get_list_peers() -> Vec<PeerInfo> {
    let contents = lines_from_file(PEERS_FILE);

    let mut all_peers = Vec::new();
    // Create the structure with ip and port
    for entry in contents.iter() {
        let split: Vec<&str> = entry.split(",").into_iter().map(|x| x.trim()).collect();
        all_peers.push(PeerInfo {
            ip: split[0].to_string(),
            port: split[1].parse::<u16>().unwrap_or(DEFAULT_PORT),
        });
    }

    // Always add 127.0.0.1 with a default port 7777
    all_peers.push(PeerInfo {
        ip: DEFAULT_IP.to_string(),
        port: DEFAULT_PORT,
    });

    return all_peers;
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename);
    match file {
        Ok(file_content) => {
            let buf = BufReader::new(file_content);
            return buf
                .lines()
                .map(|l| l.expect("Could not parse line"))
                .collect();
        }
        Err(_e) => Vec::new(),
    }
}
