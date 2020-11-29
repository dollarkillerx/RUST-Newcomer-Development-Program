pub mod error;

pub type Result<T> = std::result::Result<T,error::CommonError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
