// src/percabangan.rs

pub fn run() {
    let angka = 7;

    // IF
    if angka > 5 {
        println!("Angka lebih besar dari 5");
    } else if angka == 5 {
        println!("Angka sama dengan 5");
    } else {
        println!("Angka lebih kecil dari 5");
    }

    // MATCH
    let hari = 3;
    match hari {
        1 => println!("Senin"),
        2 => println!("Selasa"),
        3 => println!("Rabu"),
        4 => println!("Kamis"),
        5 => println!("Jumat"),
        6 | 7 => println!("Weekend 😎"),
        _ => println!("Hari tidak valid"),
    }
}
