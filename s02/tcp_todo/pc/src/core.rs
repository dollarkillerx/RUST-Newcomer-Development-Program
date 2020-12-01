pub(crate) struct Core {
    listen_addr: String,
    caesar_addr: String,
}

impl Core {
    pub(crate) fn new(listen_addr: String, caesar_addr: String) -> Self {
        Core {
            listen_addr,
            caesar_addr,
        }
    }
}