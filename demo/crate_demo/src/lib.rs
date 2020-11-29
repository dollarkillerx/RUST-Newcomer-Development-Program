pub mod conf;
mod environment;
mod err;

mod test {
    use std::fs::File;

    #[test]
 fn read_file() -> Result<File, _> {
    // File::open("Cargo.toml".parse().unwrap()).map_err(|_| )
 }
}