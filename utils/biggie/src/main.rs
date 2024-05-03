fn main() {
    if let Err(e) = biggie::get_args().and_then(biggie::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
