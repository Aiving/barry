use tokio::time::{sleep, Duration};

use crate::{
    styling::{border::BorderRadius, style::Style, thickness::Thickness, StyleExt},
    theme,
};
use relm4::{Component, ComponentParts, ComponentSender, RelmContainerExt};

pub struct DateTime {
    data: glib::DateTime,
}

pub struct DateTimeWidgets {
    date: gtk::Label,
    time: gtk::Label,
}

impl Component for DateTime {
    type Root = gtk::Box;
    type Widgets = DateTimeWidgets;

    type CommandOutput = glib::DateTime;
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
        let model = Self {
            data: glib::DateTime::now_local().unwrap(),
        };

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    loop {
                        sleep(Duration::from_secs(5)).await;

                        out.send(glib::DateTime::now_local().unwrap()).unwrap();
                    }
                })
                .drop_on_shutdown()
        });

        let time = gtk::Label::new(Some(&model.data.format("%H:%M").unwrap()));
        let date = gtk::Label::new(Some(&model.data.format("%m.%d.%Y").unwrap()));

        time.set_style(
            Style::new()
                .background_color(theme().tertiary_container)
                .color(theme().on_tertiary_container)
                .padding(&Thickness::Custom(0, 4, 0, 4))
                .border_radius(&BorderRadius::Custom(12, 4, 4, 12)),
        );

        date.set_style(
            Style::new()
                .background_color(theme().tertiary_container)
                .color(theme().on_tertiary_container)
                .padding(&Thickness::Custom(0, 4, 0, 4))
                .border_radius(&BorderRadius::Custom(4, 12, 12, 4)),
        );

        root.container_add(&time);
        root.container_add(&date);

        ComponentParts {
            model,
            widgets: Self::Widgets { date, time },
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        _: ComponentSender<Self>,
        _: &Self::Root,
    ) {
        widgets.time.set_label(&message.format("%H:%M").unwrap());
        widgets.date.set_label(&message.format("%m.%d.%Y").unwrap());

        self.data = message;
    }
}
