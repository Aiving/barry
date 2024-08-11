use crate::{
    components::{
        current_track::CurrentTrack, date_time::DateTime, metric::Metrics, volume::Volume,
    },
    data::workspace::Workspace,
    styling::{
        border::{Border, BorderRadius, BorderStyle},
        style::Style,
        thickness::Thickness,
        StyleExt,
    },
    theme,
    utils::get_display_geometry,
};

use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use hyprland::event_listener::EventListener;
use relm4::{
    factory::FactoryVecDeque, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, RelmContainerExt,
};

pub struct Bar {
    workspaces: FactoryVecDeque<Workspace>,
    current_track: Controller<CurrentTrack>,
    volume: Controller<Volume>,
    metrics: Controller<Metrics>,
    date_time: Controller<DateTime>,
}

#[derive(Debug)]
pub enum Message {
    ChangedWorkspace,
}

pub struct BarWidgets {}

impl Component for Bar {
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
    fn init(
        (): Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut workspaces = FactoryVecDeque::builder().launch_default().detach();
        let mut guard = workspaces.guard();

        for group in Workspace::all() {
            guard.push_back(group);
        }

        guard.drop();

        let current_track = CurrentTrack::builder().launch(()).detach();
        let volume = Volume::builder().launch(()).detach();
        let metrics = Metrics::builder().launch(()).detach();
        let date_time = DateTime::builder().launch(()).detach();

        let model = Self {
            workspaces,
            current_track,
            volume,
            metrics,
            date_time,
        };

        let geometry = get_display_geometry();

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    let mut event_listener = EventListener::new();

                    {
                        let out = out.clone();

                        event_listener.add_workspace_added_handler(move |_| {
                            out.send(Message::ChangedWorkspace).unwrap();
                        });
                    }

                    {
                        let out = out.clone();

                        event_listener.add_workspace_destroy_handler(move |_| {
                            out.send(Message::ChangedWorkspace).unwrap();
                        });
                    }

                    {
                        let out = out.clone();

                        event_listener.add_workspace_change_handler(move |_| {
                            out.send(Message::ChangedWorkspace).unwrap();
                        });
                    }

                    if event_listener.start_listener_async().await.is_err() {
                        println!("warning: failed to start hyprland event listener, workspaces will not work");
                    }
                })
                .drop_on_shutdown()
        });

        let workspaces_box = model.workspaces.widget();

        window.set_default_width(geometry.width());
        window.init_layer_shell();
        window.set_layer(Layer::Top);
        window.auto_exclusive_zone_enable();
        window.set_anchor(Edge::Top, true);

        let bar = gtk::CenterBox::default();

        bar.set_css_classes(&["bar"]);
        bar.set_orientation(gtk::Orientation::Horizontal);
        bar.set_style(
            Style::new()
                .margin(&Thickness::Custom(10, 10, 5, 10))
                .background_color(theme().surface_container)
                .border(
                    &Border::default()
                        .thickness(1)
                        .style(BorderStyle::Solid)
                        .color(theme().primary_container),
                )
                .border_radius(&BorderRadius::All(32))
                .padding(&Thickness::All(4))
                .min_height(26)
                .box_shadow(vec![0, 0, 4], theme().primary_container)
                .font_family("JetBrainsMono Nerd Font"),
        );

        workspaces_box.set_css_classes(&["workspaces"]);
        workspaces_box.set_orientation(gtk::Orientation::Horizontal);
        workspaces_box.set_spacing(2);
        workspaces_box.set_style(
            Style::new()
                .background_color(theme().surface)
                .margin(&Thickness::Right(4))
                .border_radius(&BorderRadius::All(32))
                .padding(&Thickness::All(4))
                .min_height(8),
        );

        let right = gtk::Box::default();

        right.set_orientation(gtk::Orientation::Horizontal);
        right.set_spacing(4);
        right.set_halign(gtk::Align::End);

        right.container_add(model.volume.widget());
        right.container_add(model.metrics.widget());
        right.container_add(model.date_time.widget());

        bar.set_start_widget(Some(workspaces_box));
        bar.set_center_widget(Some(model.current_track.widget()));
        bar.set_end_widget(Some(&right));

        window.container_add(&bar);

        let widgets = Self::Widgets {};

        ComponentParts { model, widgets }
    }

    fn update_cmd_with_view(
        &mut self,
        _: &mut Self::Widgets,
        message: Self::CommandOutput,
        _: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        let mut guard = self.workspaces.guard();

        match message {
            Message::ChangedWorkspace => {
                for (index, workspace) in Workspace::all().into_iter().enumerate() {
                    if let Some(mutable_workspace) = guard.get_mut(index) {
                        *mutable_workspace = workspace;
                    }
                }
            }
        }
    }
}
