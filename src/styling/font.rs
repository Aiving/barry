use std::fmt;

pub enum FontWeight {
    Thin,
    Normal,
    Bold,
}

impl fmt::Display for FontWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Thin => f.write_str("thin"),
            Self::Normal => f.write_str("normal"),
            Self::Bold => f.write_str("bold"),
        }
    }
}
