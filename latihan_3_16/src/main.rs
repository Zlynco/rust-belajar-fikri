mod ownership;
mod borrowing;
mod string_vs_str;
mod lifetime;

fn main() {
    ownership::run();
    borrowing::run();
    string_vs_str::run();
    lifetime::run();
}
