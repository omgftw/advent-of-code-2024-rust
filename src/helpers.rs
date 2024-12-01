pub fn is_debug() -> bool {
    std::env::var("RUST_LOG").unwrap_or_default() == "debug"
}
