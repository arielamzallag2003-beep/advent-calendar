use std::fs::File;
use std::io::{BufReader, Read};

pub const NOM: &str = "Exercice 8: Compter les chars d'un fichier 1 GiB";

const CHEMIN: &str = "data/giga.txt";
const BUFFER: usize = 64 * 1024;

#[must_use] 
pub fn run() -> String {
    // Ouvre le fichier
    let fichier = match File::open(CHEMIN) {
        Ok(f) => f,
        Err(e) => return format!("Erreur: {e} — Lance d'abord: cargo run --release --bin gen_data"),
    };

    let mut reader = BufReader::with_capacity(BUFFER, fichier);
    let mut buf = vec![0u8; BUFFER];

    let mut nb_octets: u64 = 0;
    let mut nb_continuation: u64 = 0;

    loop {
        let lus = reader.read(&mut buf).expect("Erreur de lecture");
        if lus == 0 { break; }

        nb_octets += lus as u64;

        for &octet in &buf[..lus] {
            if octet & 0b1100_0000 == 0b1000_0000 {
                nb_continuation += 1;
            }
        }
    }

    let nb_chars = nb_octets - nb_continuation;

    format!(
        "Fichier        : {CHEMIN}\n\
         Octets totaux  : {nb_octets}\n\
         Chars Unicode  : {nb_chars}\n\
         Taille         : {:.2} MiB",
        nb_octets as f64 / (1024.0 * 1024.0)
    )
}
