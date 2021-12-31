use adw::prelude::*;
use gettextrs::gettext;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use super::pages::{ImagePageWidget, WelcomePageWidget};
use super::paginator::PaginatorWidget;
use crate::Application;

mod imp {
    use super::*;
    use crate::config;
    use adw::subclass::prelude::*;

    #[derive(Debug, Default)]
    pub struct Window {
        pub(super) paginator: PaginatorWidget,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;
    }

    impl ObjectImpl for Window {
        fn constructed(&self, widget: &Self::Type) {
            widget.set_default_size(960, 720);
            widget.set_icon_name(Some(config::APP_ID));

            // Devel Profile
            if config::PROFILE == "Devel" {
                widget.add_css_class("devel");
            }

            self.paginator.add_page(WelcomePageWidget::new());
            self.paginator.add_page(ImagePageWidget::new(
                "/org/gnome/Tour/overview.svg",
                gettext("Get an Overview"),
                gettext("Press the Super key to see open windows and apps."),
            ));

            self.paginator.add_page(ImagePageWidget::new(
                "/org/gnome/Tour/search.svg",
                gettext("Just Type to Search"),
                gettext("Type in the overview to search. Launch apps, find things."),
            ));

            self.paginator.add_page(ImagePageWidget::new(
                "/org/gnome/Tour/workspaces.svg",
                gettext("Keep on Top with Workspaces"),
                gettext("Easily organize windows with the workspaces view."),
            ));

            self.paginator.add_page(ImagePageWidget::new(
                "/org/gnome/Tour/blank.svg",
                gettext("Up/Down for the Overview"),
                gettext("On a touchpad, use three-finger vertical swipes. Try it!"),
            ));

            self.paginator.add_page(ImagePageWidget::new(
                "/org/gnome/Tour/blank.svg",
                gettext("Left/Right for Workspaces"),
                gettext("On a touchpad, use three-finger horizontal swipes. Try it!"),
            ));

            let last_page = ImagePageWidget::new(
                "/org/gnome/Tour/ready-to-go.svg",
                gettext("That's it. Have a nice day!"),
                gettext("To get more advice and tips, see the Help app."),
            );
            last_page.add_css_class("last-page");
            self.paginator.add_page(last_page);

            widget.set_content(Some(&self.paginator));
            self.parent_constructed(widget);
        }
    }
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        glib::Object::new(&[("application", app)]).unwrap()
    }

    pub fn paginator(&self) -> PaginatorWidget {
        self.imp().paginator.clone()
    }

    pub fn start_tour(&self) {
        self.imp().paginator.set_page(1);
    }

    pub fn reset_tour(&self) {
        self.imp().paginator.set_page(0);
    }
}
