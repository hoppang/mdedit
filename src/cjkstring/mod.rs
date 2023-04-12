use std::fmt;

pub struct CjkString {
    s: String,
    idx: usize,
}

impl CjkString {
    pub fn from(arg: &str) -> CjkString {
        CjkString {
            s: String::from(arg),
            idx: 0
        }
    }
}

impl fmt::Debug for CjkString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(\"{}\", {})", self.s, self.idx)
    }
}
