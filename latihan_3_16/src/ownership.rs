pub fn run() {
    let s1 = String::from("Halo");
    let s2 = s1; // s1 dimove ke s2, s1 tidak bisa dipakai lagi

    // println!("{}", s1); ❌ Error: s1 sudah tidak punya ownership

    println!("s2: {}", s2);

    let x = 5;
    let y = x; // tipe primitive disalin (copy), bukan move
    println!("x: {}, y: {}", x, y);
}
