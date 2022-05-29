fn main() {
    if let Err(err) = csvrowselector::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
