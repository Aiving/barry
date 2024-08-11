use material_colors::color::Argb;

use crate::utils::ColorExt;

use super::{border::{Border, BorderRadius}, font::FontWeight, thickness::Thickness};

#[derive(Debug, Default)]
pub struct Style {
    pub(super) properties: Vec<String>,
}

impl Style {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub(super) fn with_class_name(self, class_name: &str) -> String {
        format!("{class_name} {{ {} }}", self.properties.join("\n"))
    }

    #[must_use]
    pub fn background_image(mut self, url: &str) -> Self {
        self.properties
            .push(format!("background-image: url(\"file://{url}\");"));

        self
    }

    #[must_use]
    pub fn background_size(mut self, size: &str) -> Self {
        self.properties.push(format!("background-size: {size};"));

        self
    }

    #[must_use]
    pub fn background_repeat(mut self, repeat: &str) -> Self {
        self.properties
            .push(format!("background-repeat: {repeat};"));

        self
    }

    #[must_use]
    pub fn background_position(mut self, position: &str) -> Self {
        self.properties
            .push(format!("background-position: {position};"));

        self
    }

    #[must_use]
    pub fn background_color(mut self, color: Argb) -> Self {
        self.properties
            .push(format!("background-color: {};", color.to_rgba()));

        self
    }

    #[must_use]
    pub fn color(mut self, color: Argb) -> Self {
        self.properties.push(format!("color: {};", color.to_rgba()));

        self
    }

    #[must_use]
    pub fn border_radius(mut self, radius: &BorderRadius) -> Self {
        self.properties.push(radius.to_string());

        self
    }

    #[must_use]
    pub fn font_size(mut self, size: u32) -> Self {
        self.properties.push(format!("font-size: {size}px;"));

        self
    }

    #[must_use]
    pub fn min_width(mut self, width: u32) -> Self {
        self.properties.push(format!("min-width: {width}px;"));

        self
    }

    #[must_use]
    pub fn min_height(mut self, height: u32) -> Self {
        self.properties.push(format!("min-height: {height}px;"));

        self
    }

    #[must_use]
    pub fn min_size(mut self, size: u32) -> Self {
        self.properties.push(format!("min-width: {size}px;"));
        self.properties.push(format!("min-height: {size}px;"));

        self
    }

    #[must_use]
    pub fn margin(mut self, thickness: &Thickness) -> Self {
        self.properties.push(format!("margin{thickness}"));

        self
    }

    #[must_use]
    pub fn padding(mut self, thickness: &Thickness) -> Self {
        self.properties.push(format!("padding{thickness}"));

        self
    }

    #[must_use]
    pub fn border(mut self, border: &Border) -> Self {
        self.properties.push(format!("border: {border};"));

        self
    }

    #[must_use]
    pub fn box_shadow(mut self, length: Vec<u32>, color: Argb) -> Self {
        self.properties.push(format!(
            "box-shadow: {} {};",
            length
                .into_iter()
                .map(|value| format!("{value}px"))
                .collect::<Vec<_>>()
                .join(" "),
            color.to_rgba()
        ));

        self
    }

    #[must_use]
    pub fn font_family(mut self, family: &str) -> Self {
        self.properties.push(format!("font-family: {family};"));

        self
    }

    #[must_use]
    pub fn font_weight(mut self, weight: &FontWeight) -> Self {
        self.properties.push(format!("font-weight: {weight};"));

        self
    }

    // TODO: replace with Transition struct/enum/idk
    #[must_use]
    pub fn transition(mut self, transition: &str) -> Self {
        self.properties.push(format!("transition: {transition};"));

        self
    }
}