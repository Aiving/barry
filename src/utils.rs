use gdk::{
    prelude::{DisplayExt, MonitorExt},
    Rectangle, RGBA,
};
use gio::prelude::ListModelExt;
use glib::object::Cast;
use hyprland::shared::HyprDataActive;
use material_colors::color::Argb;
use sysinfo::System;

#[must_use]
pub fn clsx(class: Vec<(bool, &str)>) -> Vec<&str> {
    class
        .into_iter()
        .filter(|(value, _)| *value)
        .map(|(_, value)| value)
        .collect()
}

#[must_use]
pub fn get_display_geometry() -> Rectangle {
    if let Ok(monitor) = hyprland::data::Monitor::get_active() {
        Rectangle::new(
            monitor.x,
            monitor.y,
            i32::from(monitor.width),
            i32::from(monitor.height),
        )
    } else {
        let display = gdk::Display::default().unwrap();
        let monitor = display
            .monitors()
            .item(0)
            .unwrap()
            .downcast::<gdk::Monitor>()
            .unwrap();

        monitor.geometry()
    }
}

pub trait SystemExt {
    fn cpu_usage(&self) -> f64;
    fn memory_usage(&self) -> f64;
}

impl SystemExt for System {
    fn cpu_usage(&self) -> f64 {
        let cpus = self.cpus();

        cpus.iter().fold(0.0, |p, c| p + f64::from(c.cpu_usage())) / cpus.len() as f64
    }

    fn memory_usage(&self) -> f64 {
        (self.free_memory() as f64 / self.total_memory() as f64) * 100.0
    }
}

pub trait ColorExt {
    #[must_use]
    fn with_alpha(&self, alpha: f64) -> Self;
    fn to_rgba(&self) -> String;
    fn as_rgba(&self) -> RGBA;
}

impl ColorExt for Argb {
    fn with_alpha(&self, alpha: f64) -> Self {
        Self::new(
            (alpha * 255.0).round() as u8,
            self.red,
            self.green,
            self.blue,
        )
    }

    fn to_rgba(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.red,
            self.green,
            self.blue,
            f64::trunc((f64::from(self.alpha) / 255.0) * 100.0) / 100.0
        )
    }

    fn as_rgba(&self) -> RGBA {
        RGBA::new(
            f32::from(self.red) / 255.0,
            f32::from(self.green) / 255.0,
            f32::from(self.blue) / 255.0,
            f32::from(self.alpha) / 255.0,
        )
    }
}
