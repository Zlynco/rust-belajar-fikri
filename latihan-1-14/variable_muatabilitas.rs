// Variabel dan Mutabilitas

fn main() {
    // Variabel immutable (default)
    let nama = "Fikri";
    println!("Halo, nama saya: {}", nama);
    // nama = "Budi"; // ❌ Error, variabel immutable

    // Variabel mutable
    let mut usia = 23;
    println!("Usia saya: {}", usia);
    usia = 24;
    println!("Setelah ulang tahun: {}", usia);

    // Shadowing (menimpa variabel lama dengan yang baru)
    let tinggi = 170;
    let tinggi = tinggi + 10;
    println!("Tinggi sekarang: {}", tinggi);
}
