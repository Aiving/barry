use crate::{
    styling::{border::BorderRadius, style::Style, thickness::Thickness, StyleExt},
    theme,
};
use gtk::prelude::*;
use relm4::{Component, ComponentParts, ComponentSender, RelmContainerExt};
use std::process::Command;

pub struct Volume;

impl Component for Volume {
    type Root = gtk::Box;
    type Widgets = ();

    type CommandOutput = ();
    type Input = ();
    type Output = ();
    type Init = ();

    fn init_root() -> Self::Root {
        Self::Root::default()
    }

    fn init((): Self::Init, root: Self::Root, _: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = Self;

        root.set_css_classes(&["volume"]);
        root.set_orientation(gtk::Orientation::Horizontal);
        root.set_spacing(12);

        root.set_style(
            Style::new()
                .margin(&Thickness::Left(4))
                .background_color(theme().surface_container_highest)
                .color(theme().secondary)
                .min_width(100)
                .border_radius(&BorderRadius::All(32))
                .padding(&Thickness::Custom(0, 8, 0, 8)),
        );

        let icon = gtk::Label::default();

        icon.set_label("");

        let slider = gtk::Scale::default();

        slider.set_range(0.0, 1.0);
        slider.set_value(
            String::from_utf8(
                Command::new("wpctl")
                    .arg("get-volume")
                    .arg("@DEFAULT_AUDIO_SINK@")
                    .output()
                    .unwrap()
                    .stdout,
            )
            .unwrap()
            .trim()[9..12]
                .parse()
                .unwrap(),
        );
        slider.set_hexpand(true);

        slider.set_child_style(
            slider.first_child().unwrap(),
            Style::new()
                .background_color(theme().on_secondary_fixed_variant)
                .border_radius(&BorderRadius::All(50))
                .min_height(6)
                .min_width(50),
        );

        slider.set_child_style(
            slider.first_child().unwrap().first_child().unwrap(),
            Style::new()
                .background_color(theme().secondary_fixed)
                .border_radius(&BorderRadius::All(10)),
        );

        slider.connect_change_value(|_, _, value| {
            Command::new("wpctl")
                .arg("set-volume")
                .arg("@DEFAULT_AUDIO_SINK@")
                .arg(value.to_string())
                .spawn()
                .unwrap();

            glib::Propagation::Proceed
        });

        root.container_add(&icon);
        root.container_add(&slider);

        ComponentParts { model, widgets: () }
    }
}
