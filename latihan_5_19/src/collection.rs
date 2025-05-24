use std::collections::HashMap;

pub fn vec_demo() {
    let mut angka = vec![1, 2, 3];
    angka.push(4);
    println!("Isi Vec: {:?}", angka);

    for a in &angka {
        println!("Angka: {}", a);
    }
}

pub fn hashmap_demo() {
    let mut skor = HashMap::new();
    skor.insert(String::from("Fikri"), 100);
    skor.insert(String::from("Budi"), 80);

    match skor.get("Fikri") {
        Some(nilai) => println!("Skor Fikri: {}", nilai),
        None => println!("Fikri tidak ditemukan"),
    }

    for (nama, nilai) in &skor {
        println!("{}: {}", nama, nilai);
    }
}

pub fn run() {
    vec_demo();
    hashmap_demo();
}
