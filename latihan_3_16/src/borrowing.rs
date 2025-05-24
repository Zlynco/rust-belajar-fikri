fn print_nama(nama: &String) {
    println!("Nama: {}", nama);
}

fn tambah_akhir(nama: &mut String) {
    nama.push_str(" Rustacean");
}

pub fn run() {
    let nama = String::from("Fikri");
    print_nama(&nama); // borrowing dengan &

    let mut nama2 = String::from("Budi");
    tambah_akhir(&mut nama2); // borrowing dengan &mut
    println!("Nama baru: {}", nama2);
}
