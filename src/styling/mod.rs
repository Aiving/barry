use gtk::prelude::*;
use style::Style;
use stylesheet::StyleSheet;

#[allow(deprecated)]
pub mod border;
pub mod font;
pub mod style;
pub mod stylesheet;
pub mod thickness;

pub trait StyleExt {
    fn set_style(&self, style: Style);
    fn set_stylesheet(&self, stylesheet: StyleSheet);
    fn set_child_style(&self, child: gtk::Widget, style: Style);
    fn set_child_stylesheet(&self, child: gtk::Widget, stylesheet: StyleSheet);
}

impl<T: WidgetExt> StyleExt for T {
    fn set_style(&self, style: Style) {
        let provider = gtk::CssProvider::new();

        provider.load_from_string(&style.with_class_name(self.css_name().as_str()));

        #[allow(deprecated)]
        self.style_context().add_provider(&provider, 900);
    }

    fn set_stylesheet(&self, stylesheet: StyleSheet) {
        let provider = gtk::CssProvider::new();
        let data = stylesheet.with_class_name(self.css_name().as_str());

        provider.load_from_string(&data);

        #[allow(deprecated)]
        self.style_context().add_provider(&provider, 1000);
    }

    fn set_child_style(&self, child: gtk::Widget, style: Style) {
        child.set_style(style);
    }

    fn set_child_stylesheet(&self, child: gtk::Widget, stylesheet: StyleSheet) {
        child.set_stylesheet(stylesheet);
    }
}
