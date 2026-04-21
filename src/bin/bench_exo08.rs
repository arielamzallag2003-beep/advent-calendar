use std::fs::File;
use std::io::{BufReader, Read};

const CHEMIN: &str = "data/giga.txt";
const BUFFER: usize = 64 * 1024;

fn main() {
    let fichier = File::open(CHEMIN)
        .unwrap_or_else(|e| panic!("Erreur: {e}\nLance d'abord: cargo run --release --bin gen_data"));

    let mut reader = BufReader::with_capacity(BUFFER, fichier);
    let mut buf = vec![0u8; BUFFER];

    let mut nb_octets: u64 = 0;
    let mut nb_continuation: u64 = 0;

    loop {
        let lus = reader.read(&mut buf).unwrap();
        if lus == 0 { break; }
        nb_octets += lus as u64;
        for &octet in &buf[..lus] {
            if octet & 0b1100_0000 == 0b1000_0000 {
                nb_continuation += 1;
            }
        }
    }

    let nb_chars = nb_octets - nb_continuation;
    println!("{CHEMIN} → {nb_octets} octets — {nb_chars} chars Unicode");
}
