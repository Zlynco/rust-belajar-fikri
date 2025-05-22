// Variabel dan Mutabilitas

pub fn main() {
    // Variabel immutable (default)
    let nama = "Fikri";
    println!("Halo, nama saya: {}", nama);
    // nama = "Budi"; // ❌ Error, variabel immutable

    // Variabel mutable
    let mut usia = 18;
    println!("Usia saya: {}", usia);
    usia = 19;
    println!("Setelah ulang tahun: {}", usia);

    // Shadowing (menimpa variabel lama dengan yang baru)
    let tinggi = 170;
    let tinggi = tinggi + 1;
    println!("Tinggi sekarang: {}", tinggi);
}
