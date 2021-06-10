use std::{env, error::Error, fmt, fs::File, io::{BufReader}, process};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Entry {
    pub codice: String,
    pub nome: String,
    pub categoria: String,
    pub prezzo_al_consumatore: String,
    pub prezzo_al_rivenditore: String,
    pub descrizione: String,
    pub immagine: String,
    pub miniatura: String,
    pub disponibilita: u8,
    pub quantita: u8,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} - category: {} - net: {} - gross: {}", self.nome, self.categoria, self.prezzo_al_rivenditore, self.prezzo_al_consumatore)
    }
}

fn read_csv(filename: String) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(filename).expect("Cannot open input file!!");
    let reader: BufReader<File> = BufReader::new(file);
    
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(reader);
    let _entries: Vec<Entry> = Vec::new();

    for result in rdr.deserialize() {
        let entry: Entry = result?;
        //entries.push(entry);
        println!("{}", entry);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Loading entries from {}", filename);

    if let Err(err) = read_csv(filename.to_string()) {
        println!("error running readCSV: {}", err);
        process::exit(1);
    }
}
