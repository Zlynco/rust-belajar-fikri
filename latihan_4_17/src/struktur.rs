pub struct User {
    pub nama: String,
    pub umur: u32,
}

impl User {
    pub fn perkenalan(&self) {
        println!("Halo, saya {} dan umur saya {}.", self.nama, self.umur);
    }

    pub fn ulang_tahun(&mut self) {
        self.umur += 1;
        println!("Selamat ulang tahun! Sekarang umur: {}", self.umur);
    }
}

pub fn run() {
    let mut user = User {
        nama: String::from("Fikri"),
        umur: 23,
    };

    user.perkenalan();
    user.ulang_tahun();
}
