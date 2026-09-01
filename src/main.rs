use rust_template::{cli, run};

fn main() {
    if std::env::args().nth(1).as_deref() == Some("repository") {
        std::process::exit(cli());
    }

    let name = std::env::args().nth(1);
    println!("{}", run(name.as_deref()));
}
