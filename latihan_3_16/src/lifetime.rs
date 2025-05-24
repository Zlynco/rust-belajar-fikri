// Fungsi ini mengembalikan referensi dari input
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn run() {
    let a = String::from("Rustacean");
    let b = String::from("Ferris");
    let hasil = longest(&a, &b);

    println!("Teks terpanjang: {}", hasil);
}
