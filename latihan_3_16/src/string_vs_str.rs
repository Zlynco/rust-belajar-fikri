pub fn run() {
    let kalimat_static: &str = "Ini &str"; // string literal
    let mut kalimat_string: String = String::from("Ini String");

    println!("&str: {}", kalimat_static);
    kalimat_string.push_str (" bisa diubah");
    println!("String: {}", kalimat_string);

    // Fungsi menerima keduanya bisa pakai: fn cetak(s: &str)
}
