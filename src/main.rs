#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cargo_common_metadata,
    clippy::too_many_lines,
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::multiple_crate_versions,
    clippy::unreadable_literal
)]

pub mod apps;
pub mod components;
pub mod data;
pub mod mpris;
pub mod styling;
pub mod utils;
pub mod widgets;

use apps::{app_search::AppSearch, bar::Bar};
use material_colors::{color::Argb, scheme::Scheme, theme::ThemeBuilder};
use relm4::{once_cell::sync::OnceCell, RelmApp};
use std::{env, str::FromStr};

static THEME: OnceCell<Scheme> = OnceCell::new();

pub fn theme() -> &'static Scheme {
    THEME.get().unwrap()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    args.iter()
        .position(|arg| arg == "--theme")
        .map_or_else(
            || {
                THEME.set(
                    ThemeBuilder::with_source(Argb::from_u32(0x2C563E))
                        .build()
                        .schemes
                        .dark,
                )
            },
            |position| {
                args.remove(position);

                let color = args.remove(position);

                THEME.set(
                    ThemeBuilder::with_source(Argb::from_str(&color).unwrap())
                        .build()
                        .schemes
                        .dark,
                )
            },
        )
        .unwrap();

    let app_name = args.first().map_or_else(
        || panic!("app name requiered"),
        |app| format!("kz.aiving.{app}"),
    );

    let app = RelmApp::new(&app_name).with_args(args);

    let display = gdk::Display::default().unwrap();
    let provider = gtk::CssProvider::new();

    provider.load_from_string("* { all: unset; }");

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 200,
    );

    if app_name == "kz.aiving.bar" {
        app.run::<Bar>(());
    } else if app_name == "kz.aiving.app-search" {
        app.run::<AppSearch>(());
    } else {
        panic!("there is no app called {app_name}");
    }
}
