use crate::utils::i18n_f;
use gettextrs::gettext;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Default, Debug)]
    pub struct WelcomePageWidget {}

    #[glib::object_subclass]
    impl ObjectSubclass for WelcomePageWidget {
        const NAME: &'static str = "WelcomePageWidget";
        type ParentType = gtk::Box;
        type Type = super::WelcomePageWidget;
    }

    impl ObjectImpl for WelcomePageWidget {
        fn constructed(&self, widget: &Self::Type) {
            let layout_manager = widget
                .layout_manager()
                .map(|l| l.downcast::<gtk::BoxLayout>().unwrap())
                .unwrap();
            layout_manager.set_orientation(gtk::Orientation::Vertical);

            let container = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(0)
                .hexpand(true)
                .vexpand(true)
                .valign(gtk::Align::Center)
                .halign(gtk::Align::Center)
                .margin_top(24)
                .margin_bottom(24)
                .build();
            widget.add_css_class("page");
            widget.add_css_class("welcome-page");

            let clamp = adw::Clamp::new();
            clamp.set_child(Some(&container));

            let logo = gtk::Picture::builder()
                .can_shrink(false)
                .keep_aspect_ratio(true)
                .build();
            logo.set_resource(Some("/org/gnome/Tour/welcome.svg"));
            container.append(&logo);

            let title = gtk::Label::new(Some(&gettext("Start the Tour")));
            title.set_margin_top(36);
            title.add_css_class("title-1");
            container.append(&title);

            let name = glib::os_info("NAME").unwrap_or_else(|| "GNOME".into());
            let version = glib::os_info("VERSION").unwrap_or_else(|| "".into());
            // Translators: The following string is formated as "Learn about new and essential features in GNOME 3.36" for example
            let text = gtk::Label::new(Some(&i18n_f(
                "Learn about the key features in {} {}.",
                &[&name, &version],
            )));
            text.add_css_class("body");
            text.set_margin_top(12);
            container.append(&text);

            widget.append(&clamp);
            self.parent_constructed(widget);
        }
    }
    impl WidgetImpl for WelcomePageWidget {}
    impl BoxImpl for WelcomePageWidget {}
}

glib::wrapper! {
    pub struct WelcomePageWidget(ObjectSubclass<imp::WelcomePageWidget>)
        @extends gtk::Widget, gtk::Box;
}

impl WelcomePageWidget {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }
}
