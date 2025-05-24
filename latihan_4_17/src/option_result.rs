pub fn cari_angka(index: usize) -> Option<i32> {
    let angka = [10, 20, 30];
    angka.get(index).copied()
}

pub fn bagi(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Tidak bisa dibagi 0"))
    } else {
        Ok(a / b)
    }
}

pub fn run() {
    // Option
    match cari_angka(1) {
        Some(nilai) => println!("Ditemukan: {}", nilai),
        None => println!("Index tidak valid"),
    }

    // Result
    match bagi(10, 2) {
        Ok(hasil) => println!("Hasil pembagian: {}", hasil),
        Err(e) => println!("Error: {}", e),
    }
}
