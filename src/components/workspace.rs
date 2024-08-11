use crate::{
    data::workspace::{Workspace, WorkspacePosition},
    styling::{border::BorderRadius, style::Style, stylesheet::StyleSheet, StyleExt},
    theme,
    utils::{clsx, ColorExt},
};
use gtk::prelude::*;
use relm4::{
    factory::{DynamicIndex, FactoryComponent, FactoryView},
    FactorySender,
};

pub struct WorkspaceWidgets {
    workspace: gtk::Label,
}

impl FactoryComponent for Workspace {
    type Init = Self;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;
    type Index = DynamicIndex;
    type Root = gtk::Label;
    type Widgets = WorkspaceWidgets;

    fn init_root(&self) -> Self::Root {
        Self::Root::default()
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        workspace: Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        _sender: FactorySender<Self>,
    ) -> Self::Widgets {
        workspace.set_label(&self.id.to_string());
        workspace.set_css_classes(&clsx(vec![
            (self.exists, "exists"),
            (self.active, "active"),
            (
                matches!(
                    self.position,
                    WorkspacePosition::First | WorkspacePosition::Both
                ),
                "first",
            ),
            (
                matches!(
                    self.position,
                    WorkspacePosition::Last | WorkspacePosition::Both
                ),
                "last",
            ),
        ]));
        workspace.set_stylesheet(
            StyleSheet::new()
                .default_style(
                    Style::new()
                        .transition("background-color 0.3s, color 0.3s, border-radius 0.3s")
                        .min_width(19)
                        .font_size(12)
                        .color(theme().secondary.with_alpha(0.5))
                        .background_color(theme().on_secondary.with_alpha(0.5)),
                )
                .style_for(
                    ".exists",
                    Style::new()
                        .color(theme().secondary)
                        .background_color(theme().on_secondary),
                )
                .style_for(
                    ".first",
                    Style::new()
                        .border_radius(&BorderRadius::TopLeft(32))
                        .border_radius(&BorderRadius::BottomLeft(32)),
                )
                .style_for(
                    ".last",
                    Style::new()
                        .border_radius(&BorderRadius::TopRight(32))
                        .border_radius(&BorderRadius::BottomRight(32)),
                )
                .style_for(
                    ".active",
                    Style::new()
                        .color(theme().primary_container)
                        .background_color(theme().primary),
                ),
        );

        Self::Widgets { workspace }
    }

    fn init_model(
        workspace: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        workspace
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: FactorySender<Self>) {
        widgets.workspace.set_css_classes(&clsx(vec![
            (self.exists, "exists"),
            (self.active, "active"),
            (
                matches!(
                    self.position,
                    WorkspacePosition::First | WorkspacePosition::Both
                ),
                "first",
            ),
            (
                matches!(
                    self.position,
                    WorkspacePosition::Last | WorkspacePosition::Both
                ),
                "last",
            ),
        ]));
    }
}
