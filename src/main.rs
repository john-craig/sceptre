use rust_template::run;

fn main() {
    let name = std::env::args().nth(1);
    println!("{}", run(name.as_deref()));
}
