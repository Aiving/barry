#![allow(deprecated, clippy::semicolon_if_nothing_returned)]

use glib::{object_subclass, prelude::*, wrapper, Properties};
use gtk::{prelude::*, subclass::prelude::*};
use std::cell::RefCell;

wrapper! {
    pub struct CircProg(ObjectSubclass<CircProgPriv>)
    @extends gtk::Widget;
}

#[derive(Properties)]
#[properties(wrapper_type = CircProg)]
pub struct CircProgPriv {
    #[property(
        get,
        set,
        nick = "Starting at",
        blurb = "Starting at",
        minimum = 0f64,
        maximum = 100f64,
        default = 0f64
    )]
    start_at: RefCell<f64>,

    #[property(get, set, nick = "Background Color", blurb = "Background color!")]
    background_color: RefCell<gdk::RGBA>,

    #[property(get, set, nick = "Value", blurb = "The value", default = 0f64)]
    value: RefCell<f64>,

    #[property(
        get,
        set,
        nick = "Thickness",
        blurb = "Thickness",
        minimum = 0f64,
        maximum = 100f64,
        default = 1f64
    )]
    thickness: RefCell<f64>,

    #[property(get, set, nick = "Clockwise", blurb = "Clockwise", default = true)]
    clockwise: RefCell<bool>,

    #[property(get, set, nick = "Child", blurb = "Child")]
    child: RefCell<Option<gtk::Widget>>,
}

// This should match the default values from the ParamSpecs
impl Default for CircProgPriv {
    fn default() -> Self {
        Self {
            start_at: RefCell::new(0.0),
            value: RefCell::new(0.0),
            thickness: RefCell::new(1.0),
            clockwise: RefCell::new(true),
            child: RefCell::new(None),
            background_color: RefCell::new(gdk::RGBA::TRANSPARENT),
        }
    }
}

impl ObjectImpl for CircProgPriv {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "value" => {
                self.value.replace(value.get().unwrap());
                self.obj().queue_draw(); // Queue a draw call with the updated value
            }
            "thickness" => {
                self.thickness.replace(value.get().unwrap());
            }
            "start-at" => {
                self.start_at.replace(value.get().unwrap());
            }
            "clockwise" => {
                self.clockwise.replace(value.get().unwrap());
            }
            "child" => {
                let child: Option<gtk::Widget> = value.get().unwrap();

                if let Some(child) = &child {
                    child.set_parent(&*self.obj());
                }

                self.child.replace(child);
            }
            "background-color" => {
                self.background_color.replace(value.get().unwrap());
            }
            x => panic!("Tried to set inexistant property of CircProg: {x}"),
        }
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }

    fn dispose(&self) {
        if let Some(child) = self.child.borrow_mut().take() {
            child.unparent();
        }
    }
}

#[object_subclass]
impl ObjectSubclass for CircProgPriv {
    type ParentType = gtk::Widget;
    type Type = CircProg;

    const NAME: &'static str = "CircProg";

    fn class_init(class: &mut Self::Class) {
        class.set_layout_manager_type::<gtk::BinLayout>();

        class.set_css_name("circular-progress");
    }
}

impl Default for CircProg {
    fn default() -> Self {
        Self::new()
    }
}

impl CircProg {
    #[must_use]
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}

impl WidgetImpl for CircProgPriv {
    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        let total_width = self.obj().width() as f32;
        let total_height = self.obj().height() as f32;

        let cr = snapshot.append_cairo(&gtk::graphene::Rect::new(
            0.0,
            0.0,
            total_width,
            total_height,
        ));

        let total_width = f64::from(total_width);
        let total_height = f64::from(total_height);

        let value = *self.value.borrow();
        let start_at = *self.start_at.borrow();
        let thickness = *self.thickness.borrow();
        let clockwise = *self.clockwise.borrow();

        let styles = self.obj().style_context();
        let margin = styles.margin();
        // Padding is not supported yet
        let fg_color: gdk::RGBA = styles.color();
        let bg_color: gdk::RGBA = self.background_color.get(|color| *color);
        let (start_angle, end_angle) = if clockwise {
            (0.0, perc_to_rad(value))
        } else {
            (perc_to_rad(100.0 - value), 2f64 * std::f64::consts::PI)
        };

        let center = (total_width / 2.0, total_height / 2.0);

        let circle_width = total_width - f64::from(margin.left()) - f64::from(margin.right());
        let circle_height = total_height - f64::from(margin.top()) - f64::from(margin.bottom());
        let outer_ring = f64::min(circle_width, circle_height) / 2.0;
        let inner_ring = (f64::min(circle_width, circle_height) / 2.0) - thickness;

        // Draw the children widget, clipping it to the inside
        if let Some(child) = &*self.child.borrow() {
            cr.save().unwrap();

            // Center circular clip
            cr.arc(
                center.0,
                center.1,
                inner_ring + 1.0,
                0.0,
                perc_to_rad(100.0),
            );
            cr.set_source_rgba(
                f64::from(bg_color.red()),
                0.0,
                0.0,
                f64::from(bg_color.alpha()),
            );
            cr.clip();

            // cr.show_text("").unwrap();

            // Children widget
            self.obj().snapshot_child(child, snapshot);

            cr.reset_clip();
            cr.restore().unwrap();
        }

        cr.save().unwrap();

        // Centering
        cr.translate(center.0, center.1);
        cr.rotate(perc_to_rad(start_at));
        cr.translate(-center.0, -center.1);

        // Background Ring
        cr.move_to(center.0, center.1);
        cr.arc(center.0, center.1, outer_ring, 0.0, perc_to_rad(100.0));
        cr.set_source_rgba(
            f64::from(bg_color.red()),
            f64::from(bg_color.green()),
            f64::from(bg_color.blue()),
            f64::from(bg_color.alpha()),
        );
        cr.move_to(center.0, center.1);
        cr.arc(center.0, center.1, inner_ring, 0.0, perc_to_rad(100.0));
        cr.set_fill_rule(gtk::cairo::FillRule::EvenOdd); // Substract one circle from the other
        cr.fill().unwrap();

        // Foreground Ring
        cr.move_to(center.0, center.1);
        cr.arc(center.0, center.1, outer_ring, start_angle, end_angle);
        cr.set_source_rgba(
            f64::from(fg_color.red()),
            f64::from(fg_color.green()),
            f64::from(fg_color.blue()),
            f64::from(fg_color.alpha()),
        );
        cr.move_to(center.0, center.1);
        cr.arc(center.0, center.1, inner_ring, start_angle, end_angle);
        cr.set_fill_rule(gtk::cairo::FillRule::EvenOdd); // Substract one circle from the other
        cr.fill().unwrap();
        cr.restore().unwrap();
    }
}

fn perc_to_rad(n: f64) -> f64 {
    (n / 100f64) * 2f64 * std::f64::consts::PI
}
