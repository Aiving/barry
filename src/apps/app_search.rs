#![allow(dead_code)]

use crate::{
    styling::{
        border::{Border, BorderRadius, BorderStyle},
        style::Style,
        stylesheet::StyleSheet,
        thickness::Thickness,
        StyleExt,
    },
    theme,
    utils::get_display_geometry,
};

use gtk::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use relm4::{
    factory::{DynamicIndex, FactoryComponent, FactoryVecDeque, FactoryView},
    Component, ComponentParts, ComponentSender, FactorySender, RelmContainerExt,
};

#[derive(Debug)]
struct App {
    info: gio::AppInfo,
    name: String,
    icon: Option<String>,
}

impl App {
    fn all() -> Vec<Self> {
        gio::AppInfo::all()
            .into_iter()
            .map(|app| Self {
                name: app.display_name().to_string(),
                icon: app
                    .icon()
                    .and_then(|icon| icon.to_string().map(|icon| icon.to_string())),
                info: app,
            })
            .collect::<Vec<_>>()
    }

    fn launch(&self) {
        self.info
            .launch(&[], None::<&gio::AppLaunchContext>)
            .unwrap();
    }
}

pub struct AppWidgets {}

impl FactoryComponent for App {
    type Init = Self;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;
    type Index = DynamicIndex;
    type Root = gtk::Box;
    type Widgets = AppWidgets;

    fn init_root(&self) -> Self::Root {
        Self::Root::default()
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        container: Self::Root,
        returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let controller = gtk::EventControllerKey::new();

        controller.connect_key_pressed(|_, key, _, _| {
            println!("{key}");

            glib::Propagation::Proceed
        });

        container.set_orientation(gtk::Orientation::Horizontal);
        container.set_spacing(4);
        returned_widget.set_stylesheet(
            StyleSheet::new()
                .default_style(
                    Style::new()
                        .background_color(theme().surface_container_highest)
                        .border_radius(&BorderRadius::All(4))
                        .font_size(12)
                        .margin(&Thickness::Custom(2, 0, 2, 0))
                        .padding(&Thickness::All(4))
                        .transition("background-color .3s"),
                )
                .style_for(
                    ":focus-within",
                    Style::new()
                        .background_color(theme().primary_container)
                        .color(theme().on_primary_container),
                )
                .style_for(
                    ":first-child",
                    Style::new().border_radius(&BorderRadius::Custom(8, 8, 4, 4)),
                )
                .style_for(
                    ":last-child",
                    Style::new().border_radius(&BorderRadius::Custom(4, 4, 8, 8)),
                ),
        );
        returned_widget.add_controller(controller);

        let icon = gtk::Image::from_icon_name(self.icon.as_ref().unwrap_or(&String::new()));
        let name = gtk::Label::new(Some(&self.name));

        container.container_add(&icon);
        container.container_add(&name);

        Self::Widgets {}
    }

    fn init_model(app: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        app
    }

    // fn update_view(&self, widgets: &mut Self::Widgets, _sender: FactorySender<Self>) {}
}

pub struct AppSearch {
    applications: FactoryVecDeque<App>,
}

#[derive(Debug)]
pub enum Message {}

pub struct BarWidgets {}

impl Component for AppSearch {
    type Init = ();
    type Input = ();
    type Output = ();
    type Root = gtk::ApplicationWindow;
    type Widgets = BarWidgets;
    type CommandOutput = Message;

    fn init_root() -> Self::Root {
        Self::Root::default()
    }

    // Initialize the component.
    fn init((): Self::Init, window: Self::Root, _: ComponentSender<Self>) -> ComponentParts<Self> {
        let mut applications = FactoryVecDeque::builder().launch_default().detach();
        let mut guard = applications.guard();

        for group in App::all() {
            guard.push_back(group);
        }

        guard.drop();

        let model = Self { applications };

        let geometry = get_display_geometry();

        window.set_default_size(320, geometry.height());
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::OnDemand);
        window.auto_exclusive_zone_enable();
        window.set_anchor(Edge::Left, true);

        let container = gtk::Box::default();

        container.set_orientation(gtk::Orientation::Vertical);
        container.set_style(
            Style::new()
                .margin(&Thickness::Custom(10, 10, 10, 10))
                .background_color(theme().surface_container)
                .border(
                    &Border::default()
                        .thickness(1)
                        .style(BorderStyle::Solid)
                        .color(theme().primary_container),
                )
                .border_radius(&BorderRadius::All(8))
                .padding(&Thickness::All(4))
                .box_shadow(vec![0, 0, 4], theme().primary_container)
                .font_family("JetBrainsMono Nerd Font"),
        );

        let search = gtk::SearchEntry::default();

        search.set_style(
            Style::new()
                .background_color(theme().surface_container_highest)
                .border_radius(&BorderRadius::All(12))
                .padding(&Thickness::All(4))
                .margin(&Thickness::Bottom(2)),
        );

        let apps = model.applications.widget();

        apps.set_selection_mode(gtk::SelectionMode::Single);

        let scroller = gtk::ScrolledWindow::default();

        scroller.set_child_style(
            scroller.first_child().unwrap(),
            Style::new().border_radius(&BorderRadius::All(8)),
        );

        scroller.set_child(Some(apps));
        scroller.set_vexpand(true);

        container.container_add(&search);
        container.container_add(&scroller);

        window.container_add(&container);

        let widgets = Self::Widgets {};

        ComponentParts { model, widgets }
    }

    fn update_cmd_with_view(
        &mut self,
        _: &mut Self::Widgets,
        _: Self::CommandOutput,
        _: ComponentSender<Self>,
        _: &Self::Root,
    ) {
    }
}
