fn main() {
    if let Err(e) = rhead::get_args().and_then(rhead::run) {
        eprint!("{}", e);
        std::process::exit(1);
    }
}
