pub enum Cuaca {
    Cerah,
    Hujan,
    Berawan,
}

pub fn info_cuaca(c: Cuaca) {
    match c {
        Cuaca::Cerah => println!("Cuaca cerah, ayo keluar!"),
        Cuaca::Hujan => println!("Bawa payung, ya."),
        Cuaca::Berawan => println!("Mendung, mungkin hujan nanti."),
    }
}

pub fn run() {
    let hari_ini = Cuaca::Cerah;
    info_cuaca(hari_ini);
}
