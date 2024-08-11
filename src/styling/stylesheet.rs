use super::style::Style;

#[derive(Default)]
pub struct StyleSheet {
    pub(super) styles: Vec<(Option<String>, Style)>,
}

impl StyleSheet {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub(super)  fn with_class_name(self, class_name: &str) -> String {
        self.styles
            .into_iter()
            .map(|(selector, style)| {
                if let Some(selector) = selector {
                    format!(
                        "{class_name}{selector} {{ {} }}",
                        style.properties.join("\n")
                    )
                } else {
                    style.with_class_name(class_name)
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    #[must_use]
    pub fn default_style(mut self, style: Style) -> Self {
        self.styles.push((None, style));

        self
    }

    #[must_use]
    pub fn style_for(mut self, selector: &str, style: Style) -> Self {
        self.styles.push((Some(selector.to_string()), style));

        self
    }
}
