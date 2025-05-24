mod collection;
mod err_handling;

fn main() {
    println!("===== Koleksi =====");
    collection::run();

    println!("\n===== Error Handling =====");
    err_handling::error_demo();
}
