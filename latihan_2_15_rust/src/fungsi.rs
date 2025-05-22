// src/fungsi.rs

fn sapa(nama: &str) {
    println!("Halo, {}!", nama);
}

fn tambah(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run() {
    sapa("Fikri");

    let hasil = tambah(5, 3);
    println!("Hasil tambah: {}", hasil);
}
