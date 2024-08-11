use crate::utils::{ColorExt, SystemExt};
use crate::{
    styling::{border::BorderRadius, style::Style, thickness::Thickness, StyleExt},
    theme,
    widgets::CircularProgress,
};
use gtk::prelude::*;
use relm4::{Component, ComponentParts, ComponentSender, RelmContainerExt};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::time::{sleep, Duration};

pub struct Metrics {
    system: System,
    cpu: f64,
    memory: f64,
}

pub struct MetricsWidgets {
    cpu: CircularProgress,
    memory: CircularProgress,
}

impl Component for Metrics {
    type Root = gtk::Box;
    type Widgets = MetricsWidgets;

    type CommandOutput = ();
    type Input = ();
    type Output = ();
    type Init = ();

    fn init_root() -> Self::Root {
        Self::Root::new(gtk::Orientation::Horizontal, 2)
    }

    fn init(
        (): Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let system = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::new().with_ram()),
        );

        let model = Self {
            cpu: system.cpu_usage(),
            memory: system.memory_usage(),
            system,
        };

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    loop {
                        sleep(Duration::from_secs(2)).await;

                        out.send(()).unwrap();
                    }
                })
                .drop_on_shutdown()
        });

        let (cpu_container, cpu) = view(&MetricKind::Cpu, model.cpu);
        let (memory_container, memory) = view(&MetricKind::Memory, model.memory);

        root.container_add(&cpu_container);
        root.container_add(&memory_container);

        ComponentParts {
            model,
            widgets: Self::Widgets { cpu, memory },
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        (): Self::CommandOutput,
        _: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();

        let (cpu, memory) = (self.system.cpu_usage(), self.system.memory_usage());

        widgets.cpu.set_value(cpu);
        widgets.memory.set_value(memory);

        self.cpu = cpu;
        self.memory = memory;
    }
}

pub enum MetricKind {
    Cpu,
    Memory,
}

fn view(kind: &MetricKind, value: f64) -> (gtk::Box, CircularProgress) {
    let container = gtk::Box::default();

    container.set_width_request(24);
    container.set_height_request(24);
    container.set_style(
        Style::new()
            .background_color(theme().surface_container_highest)
            .border_radius(&BorderRadius::All(12)),
    );

    let progress = CircularProgress::default();

    progress.set_value(value);
    progress.set_start_at(75.0);
    progress.set_thickness(2.0);
    progress.set_clockwise(true);
    progress.set_background_color(theme().primary_container.as_rgba());
    progress.set_width_request(24);
    progress.set_height_request(24);
    progress.set_style(
        Style::new()
            .color(theme().on_primary_container)
            .border_radius(&BorderRadius::All(12)),
    );

    let icon = gtk::Label::default();

    icon.set_label(match kind {
        MetricKind::Cpu => "",
        MetricKind::Memory => "",
    });

    icon.set_style(match kind {
        MetricKind::Cpu => Style::new().font_size(14).padding(&Thickness::Right(6)),
        MetricKind::Memory => Style::new().font_size(12).padding(&Thickness::Right(2)),
    });

    progress.set_child(icon);

    container.container_add(&progress);

    (container, progress)
}
