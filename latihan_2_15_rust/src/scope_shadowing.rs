// src/scope_shadowing.rs

pub fn run() {
    // Scope
    let x = 10;
    {
        let y = 20;
        println!("Dalam blok: x = {}, y = {}", x, y);
    }
    // println!("{}", y); // ❌ Error: y di luar scope

    // Shadowing
    let angka = 100;
    let angka = angka + 50;
    let angka = angka * 2;
    println!("Angka setelah shadowing: {}", angka);
}
