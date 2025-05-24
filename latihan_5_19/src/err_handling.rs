use std::fs::File;
use std::io::{self, Read};

pub fn baca_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // kalau gagal, langsung return Err
    let mut isi = String::new();
    file.read_to_string(&mut isi)?;
    Ok(isi)
}

pub fn error_demo() {
    match baca_file("contoh.txt") {
        Ok(isi) => println!("Isi file:\n{}", isi),
        Err(e) => println!("Gagal baca file: {}", e),
    }

    //panic!
    // let angka = [1, 2, 3];
    // println!("{}", angka[10]); // panic: index out of bounds
}
