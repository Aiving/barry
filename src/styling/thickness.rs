use std::fmt;

pub enum Thickness {
    All(u32),
    Custom(u32, u32, u32, u32),
    Left(u32),
    Top(u32),
    Right(u32),
    Bottom(u32),
}

impl fmt::Display for Thickness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All(radius) => write!(f, ": {radius}px;"),
            Self::Custom(top_left, top_right, bottom_left, bottom_right) => {
                write!(
                    f,
                    ": {top_left}px {top_right}px {bottom_left}px {bottom_right}px;"
                )
            }
            Self::Left(value) => write!(f, "-left: {value}px;"),
            Self::Top(value) => write!(f, "-top: {value}px;"),
            Self::Right(value) => write!(f, "-right: {value}px;"),
            Self::Bottom(value) => write!(f, "-bottom: {value}px;"),
        }
    }
}
