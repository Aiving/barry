use std::fmt;

use material_colors::color::Argb;

use crate::utils::ColorExt;

pub enum BorderRadius {
    All(u32),
    Custom(u32, u32, u32, u32),
    TopLeft(u32),
    TopRight(u32),
    BottomLeft(u32),
    BottomRight(u32),
}

#[derive(Default)]
pub enum BorderStyle {
    #[default]
    None,
    Solid,
    Inset,
    Outset,
    Hidden,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
}

#[derive(Default)]
pub struct Border {
    thickness: u32,
    style: BorderStyle,
    color: Argb,
}

impl Border {
    #[must_use]
    pub const fn thickness(mut self, thickness: u32) -> Self {
        self.thickness = thickness;

        self
    }

    #[must_use]
    pub const fn style(mut self, style: BorderStyle) -> Self {
        self.style = style;

        self
    }

    #[must_use]
    pub const fn color(mut self, color: Argb) -> Self {
        self.color = color;

        self
    }
}

impl fmt::Display for BorderRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All(radius) => write!(f, "border-radius: {radius}px;"),
            Self::Custom(top_left, top_right, bottom_left, bottom_right) => {
                write!(
                    f,
                    "border-radius: {top_left}px {top_right}px {bottom_left}px {bottom_right}px;"
                )
            }
            Self::TopLeft(radius) => write!(f, "border-top-left-radius: {radius}px;"),
            Self::TopRight(radius) => write!(f, "border-top-right-radius: {radius}px;"),
            Self::BottomLeft(radius) => write!(f, "border-bottom-left-radius: {radius}px;"),
            Self::BottomRight(radius) => {
                write!(f, "border-bottom-right-radius: {radius}px;")
            }
        }
    }
}

impl fmt::Display for Border {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}px {} {}",
            self.thickness,
            match self.style {
                BorderStyle::None => "none",
                BorderStyle::Solid => "solid",
                BorderStyle::Inset => "inset",
                BorderStyle::Outset => "outset",
                BorderStyle::Hidden => "hidden",
                BorderStyle::Dotted => "dotted",
                BorderStyle::Dashed => "dashed",
                BorderStyle::Double => "double",
                BorderStyle::Groove => "groove",
                BorderStyle::Ridge => "ridge",
            },
            self.color.to_rgba()
        )
    }
}
