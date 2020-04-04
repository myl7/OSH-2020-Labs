quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ReadFailed {
            from(std::io::Error)
        }
        BadCmd(reason: String) {
            from()
            from(s: &str) -> (s.to_string())
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
