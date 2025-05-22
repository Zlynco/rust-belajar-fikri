// src/looping.rs

pub fn run() {
    // loop (infinite loop + break)
    let mut count = 0;
    loop {
        if count == 3 {
            println!("Loop berhenti di count = {}", count);
            break;
        }
        println!("Looping ke-{}", count);
        count += 1;
    }

    // while
    let mut angka = 1;
    while angka <= 5 {
        println!("Angka (while): {}", angka);
        angka += 1;
    }

    // for
    let daftar = [10, 20, 30];
    for item in daftar.iter() {
        println!("Item: {}", item);
    }

    for i in 1..=3 {
        println!("For range ke-{}", i);
    }
}
