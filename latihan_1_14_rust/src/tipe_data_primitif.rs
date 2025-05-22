// Tipe Data Primitif

pub fn main() {
    // Integer
    let _umur: i32 = 18;
    let _jumlah: u32 = 100;

    // Float
    let _pi: f64 = 3.14159;

    // Boolean
    let _belajar: bool = true;

    // Character (1 karakter, bisa emote juga)
    let _simbol: char = '🚀';

    // Tuple (gabungan berbagai tipe)
    let data: (i32, f64, char) = (10, 2.5, 'A');
    println!("Tuple: {}, {}, {}", data.0, data.1, data.2);

    // Array (harus satu tipe)
    let warna = ["merah", "hijau", "biru"];
    println!("Warna kedua: {}", warna[1]);

    // Latihan
    let angka = [1, 2, 3, 4];
    println!("Angka ke-3: {}", angka[2]);
}
