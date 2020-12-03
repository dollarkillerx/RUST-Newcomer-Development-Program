pub struct Core {
    listen_addr: String,
    caesar_addr: String,
}

impl Core {
    fn new(listen_addr: String, caesar_addr: String) -> Core {
        Core{
            listen_addr,
            caesar_addr,
        }
    }


}