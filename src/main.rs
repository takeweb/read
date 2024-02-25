fn main() {
    if let Err(e) = rhead::run() {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
