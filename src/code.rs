use std::fmt::{Display, Formatter};

pub struct HuffmanCode {
    depth: usize,
    code: usize,
}

impl HuffmanCode {
    pub(crate) fn new() -> Self {
        Self { depth: 0, code: 0 }
    }
    pub(crate) fn left(&self) -> Self {
        Self {
            depth: self.depth + 1,
            code: self.code << 1,
        }
    }
    pub(crate) fn right(&self) -> Self {
        Self {
            depth: self.depth + 1,
            code: (self.code << 1) + 1,
        }
    }
}

impl Display for HuffmanCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:b}",
            "0".repeat(
                self.depth
                    .checked_sub(format!("{:b}", self.code).len())
                    .or(Some(0))
                    .unwrap()
            ),
            self.code
        )
    }
}