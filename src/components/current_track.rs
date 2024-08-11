use crate::data::track::Track;
use crate::mpris::{Player, Players};
use crate::styling::font::FontWeight;
use crate::utils::ColorExt;
use crate::{
    styling::{border::BorderRadius, style::Style, thickness::Thickness, StyleExt},
    theme,
    widgets::CircularProgress,
};
use gtk::prelude::*;
use material_colors::dynamic_color::Variant;
use material_colors::image::ImageReader;
use material_colors::scheme::Scheme;
use material_colors::theme::ThemeBuilder;
use relm4::{
    Component, ComponentParts, ComponentSender, RelmContainerExt, RelmRemoveAllExt, Sender,
};
use tokio_stream::StreamExt;

pub struct CurrentTrack {
    previous_track: Option<Track>,
    track: Option<Track>,
}

pub struct PopoverWidgets {
    container: gtk::Box,
    image_container: gtk::Box,
    progress: gtk::ProgressBar,
    artist: gtk::Label,
    title: gtk::Label,
}

pub struct CurrentTrackWidgets {
    container: gtk::Box,
    progress: CircularProgress,
    artist: gtk::Label,
    delimiter: gtk::Label,
    title: gtk::Label,
    popover: PopoverWidgets,
}

#[derive(Debug)]
pub enum CurrentTrackMessage {
    GotTrack(Box<(Option<Scheme>, Option<Track>)>),
    PositionChanged(i64),
    PlayerRemoved,
}

async fn process_player(out: Sender<CurrentTrackMessage>, player: Player<'_>) -> ((), ()) {
    let metadata = player.get_metadata().await.unwrap();
    let position = player.get_position().await.unwrap_or_default();

    let track = Track::new(metadata, position);

    let theme = track
        .as_ref()
        .and_then(|track| track.image.as_ref())
        .and_then(|path| ImageReader::open(path).ok())
        .map(|image| {
            ThemeBuilder::with_source(ImageReader::extract_color(&image))
                .variant(Variant::Content)
                .build()
                .schemes
                .dark
        });

    out.send(CurrentTrackMessage::GotTrack(Box::new((theme, track))))
        .unwrap();

    tokio::join!(
        async {
            let mut stream = player.metadata_stream().await;

            while let Some(event) = stream.next().await {
                let position = player.get_position().await.unwrap_or_default();

                let track = Track::new(event.get().await.unwrap(), position);

                let theme = track
                    .as_ref()
                    .and_then(|track| track.image.as_ref())
                    .and_then(|path| ImageReader::open(path).ok())
                    .map(|image| {
                        ThemeBuilder::with_source(ImageReader::extract_color(&image))
                            .variant(Variant::Content)
                            .build()
                            .schemes
                            .dark
                    });

                out.send(CurrentTrackMessage::GotTrack(Box::new((theme, track))))
                    .unwrap();
            }
        },
        async {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                if let Some(position) = player.get_position().await {
                    out.send(CurrentTrackMessage::PositionChanged(position))
                        .unwrap();
                }
            }
        }
    )
}

impl Component for CurrentTrack {
    type Root = gtk::MenuButton;
    type Widgets = CurrentTrackWidgets;

    type CommandOutput = CurrentTrackMessage;
    type Input = ();
    type Output = ();
    type Init = ();

    fn init_root() -> Self::Root {
        Self::Root::default()
    }

    fn init(
        (): Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            previous_track: None,
            track: None,
        };

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    let players = Players::new().await;
                    let stream = players.owner_changed_steam().await;

                    let mut future = Player::find_active()
                        .await
                        .map(|player| tokio::spawn(process_player(out.clone(), player)));

                    if let Some(mut stream) = stream {
                        while let Some(owner) = stream.next().await {
                            if let Ok(owner) = owner.args() {
                                if owner.new_owner.is_none() {
                                    if let Some(handle) = future.take() {
                                        handle.abort();

                                        out.send(CurrentTrackMessage::PlayerRemoved).unwrap();
                                    }
                                } else if future.is_none() {
                                    future = Player::find_active().await.map(|player| {
                                        tokio::spawn(process_player(out.clone(), player))
                                    });
                                }
                            }
                        }
                    };
                })
                .drop_on_shutdown()
        });

        let popover = gtk::Popover::default();

        let container = gtk::Box::new(gtk::Orientation::Vertical, 4);
        let sub_container = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        let image_container = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        let data_container = gtk::Box::new(gtk::Orientation::Vertical, 4);
        let artist = gtk::Label::default();
        let title = gtk::Label::default();
        let progress = gtk::ProgressBar::default();

        container.set_style(
            Style::new()
                .background_color(theme().surface_bright)
                .border_radius(&BorderRadius::All(12))
                .padding(&Thickness::All(4)),
        );

        image_container.set_style(
            Style::new()
                .border_radius(&BorderRadius::All(12))
                .min_size(48),
        );

        title.set_halign(gtk::Align::Start);
        artist.set_halign(gtk::Align::Start);

        progress.set_style(Style::new().margin(&Thickness::Custom(0, 4, 4, 4)));
        progress.set_child_style(
            progress.first_child().unwrap(),
            Style::new()
                .background_color(theme().on_primary_fixed_variant)
                .border_radius(&BorderRadius::All(3))
                .min_height(6),
        );

        progress.set_child_style(
            progress.first_child().unwrap().first_child().unwrap(),
            Style::new()
                .background_color(theme().primary_fixed)
                .border_radius(&BorderRadius::All(3))
                .min_height(6),
        );

        data_container.container_add(&title);
        data_container.container_add(&artist);

        sub_container.container_add(&image_container);
        sub_container.container_add(&data_container);

        container.container_add(&sub_container);
        container.container_add(&progress);

        popover.set_child(Some(&container));

        root.set_cursor_from_name(Some("pointer"));
        root.set_popover(Some(&popover));

        let popover = PopoverWidgets {
            container,
            image_container,
            progress,
            artist,
            title,
        };

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 4);

        container.set_visible(false);
        container.set_style(
            Style::new()
                .background_color(theme().surface_container_highest)
                .transition("background-color 1s")
                .border_radius(&BorderRadius::All(12)),
        );

        let progress = CircularProgress::new();

        progress.set_value(0.0);
        progress.set_start_at(75.0);
        progress.set_thickness(2.0);
        progress.set_clockwise(true);
        progress.set_background_color(theme().on_primary.as_rgba());
        progress.set_width_request(24);
        progress.set_height_request(24);
        progress.set_style(
            Style::new()
                .color(theme().primary)
                .border_radius(&BorderRadius::All(12))
                .min_size(24),
        );

        let image = gtk::Box::default();

        image.set_visible(false);
        image.set_style(
            Style::new()
                // .background_image(path)
                .background_size("cover")
                .background_repeat("no-repeat")
                .background_position("center")
                .border_radius(&BorderRadius::All(12))
                .margin(&Thickness::All(2))
                .min_size(20),
        );

        progress.set_child(image);

        let artist = gtk::Label::new(None);

        artist.set_style(Style::new().color(theme().secondary).transition("color 1s"));

        let delimiter = gtk::Label::new(Some("-"));

        delimiter.set_style(
            Style::new()
                .color(theme().on_surface_variant)
                .transition("color 1s"),
        );

        let title = gtk::Label::new(None);

        title.set_style(
            Style::new()
                .margin(&Thickness::Right(8))
                .color(theme().primary)
                .transition("color 1s")
                .font_weight(&FontWeight::Bold),
        );

        container.container_add(&progress);
        container.container_add(&artist);
        container.container_add(&delimiter);
        container.container_add(&title);

        root.set_child(Some(&container));

        ComponentParts {
            model,
            widgets: Self::Widgets {
                container,
                progress,
                artist,
                delimiter,
                title,
                popover,
            },
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        _: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        let Self::Widgets {
            container,
            progress,
            artist,
            delimiter,
            title,
            popover,
        } = widgets;

        match message {
            CurrentTrackMessage::GotTrack(track) => {
                if let (theme_, Some(track)) = track.as_ref() {
                    if container.is_visible()
                        && (track.id.ends_with("NoTrack") || track.duration == 0)
                    {
                        container.set_visible(false);
                    } else if !track.id.ends_with("NoTrack") && track.duration != 0
                        || self.track != self.previous_track && !track.title.is_empty()
                    {
                        #[allow(clippy::or_fun_call)]
                        let theme = theme_.as_ref().unwrap_or(theme());

                        popover.container.set_style(
                            Style::new()
                                .background_color(theme.surface_bright)
                                .border_radius(&BorderRadius::All(12))
                                .padding(&Thickness::All(4)),
                        );

                        container.set_style(
                            Style::new()
                                .background_color(theme.surface_container_highest)
                                .transition("background-color 1s")
                                .border_radius(&BorderRadius::All(12)),
                        );

                        let value = (track.position as f64 / track.duration as f64).clamp(0.0, 1.0);
                        let value = if value.is_nan() { 0.0 } else { value };

                        popover.progress.set_fraction(value);
                        popover.progress.set_child_style(
                            popover.progress.first_child().unwrap(),
                            Style::new()
                                .background_color(theme.on_secondary_fixed_variant)
                                .border_radius(&BorderRadius::All(3))
                                .min_height(6),
                        );

                        popover.progress.set_child_style(
                            popover
                                .progress
                                .first_child()
                                .unwrap()
                                .first_child()
                                .unwrap(),
                            Style::new()
                                .background_color(theme.secondary_fixed)
                                .border_radius(&BorderRadius::All(3))
                                .min_height(6),
                        );

                        progress.set_value(value * 100.0);
                        progress.set_background_color(theme.on_primary.as_rgba());
                        progress.set_style(
                            Style::new()
                                .color(theme.primary)
                                .border_radius(&BorderRadius::All(12))
                                .min_size(24),
                        );

                        let image = progress.child().unwrap();

                        if let Some(path) = track.image.as_ref() {
                            image.set_visible(true);
                            image.set_style(
                                Style::new()
                                    .background_image(path)
                                    .background_size("cover")
                                    .background_repeat("no-repeat")
                                    .background_position("center")
                                    .border_radius(&BorderRadius::All(12))
                                    .margin(&Thickness::All(2))
                                    .min_size(20),
                            );

                            let picture = gtk::Picture::for_filename(path);

                            picture.set_content_fit(gtk::ContentFit::Cover);
                            picture.set_style(
                                Style::new()
                                    .border_radius(&BorderRadius::All(12))
                                    .margin(&Thickness::All(2))
                                    .min_size(48),
                            );

                            popover.image_container.remove_all();
                            popover.image_container.container_add(&picture);
                        } else {
                            image.set_visible(false);
                            image.set_style(
                                Style::new()
                                    .background_size("cover")
                                    .background_repeat("no-repeat")
                                    .background_position("center")
                                    .border_radius(&BorderRadius::All(12))
                                    .margin(&Thickness::All(2))
                                    .min_size(20),
                            );

                            popover.image_container.remove_all();
                        }

                        popover.artist.set_label(&track.artist);
                        popover
                            .artist
                            .set_style(Style::new().color(theme.secondary).transition("color 1s"));
                        popover.title.set_label(&track.title);
                        popover.title.set_style(
                            Style::new()
                                .color(theme.primary)
                                .transition("color 1s")
                                .font_weight(&FontWeight::Bold),
                        );

                        artist.set_label(&track.artist);
                        artist
                            .set_style(Style::new().color(theme.secondary).transition("color 1s"));
                        delimiter.set_style(
                            Style::new()
                                .color(theme.on_surface_variant)
                                .transition("color 1s"),
                        );
                        title.set_label(&track.title);
                        title.set_style(
                            Style::new()
                                .margin(&Thickness::Right(8))
                                .color(theme.primary)
                                .transition("color 1s")
                                .font_weight(&FontWeight::Bold),
                        );

                        container.set_visible(true);
                    }
                } else if container.is_visible() {
                    container.set_visible(false);
                }

                self.previous_track.clone_from(&self.track);
                self.track = track.1;
            }
            CurrentTrackMessage::PositionChanged(position) => {
                if let Some(track) = self.track.as_mut() {
                    let value = (position as f64 / track.duration as f64).clamp(0.0, 1.0);
                    let value = if value.is_nan() { 0.0 } else { value };

                    popover.progress.set_fraction(value);

                    progress.set_value(value * 100.0);

                    track.position = position;
                }
            }
            CurrentTrackMessage::PlayerRemoved => {
                container.set_visible(false);
            }
        }
    }
}
