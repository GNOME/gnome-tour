use adw::prelude::*;
use gettextrs::gettext;

use super::pages::{ImagePageWidget, WelcomePageWidget};
use super::paginator::PaginatorWidget;
use crate::config::{APP_ID, PROFILE};
use crate::Application;

#[derive(Debug)]
pub struct Window {
    pub widget: adw::ApplicationWindow,
    pub paginator: PaginatorWidget,
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let widget = adw::ApplicationWindow::new(app);

        let paginator = PaginatorWidget::new();

        let mut window_widget = Window { widget, paginator };

        window_widget.init();
        window_widget
    }

    pub fn start_tour(&self) {
        self.paginator.set_page(1);
    }

    pub fn reset_tour(&self) {
        self.paginator.set_page(0);
    }

    fn init(&mut self) {
        self.widget.set_default_size(960, 720);
        self.widget.set_icon_name(Some(APP_ID));

        // Devel Profile
        if PROFILE == "Devel" {
            self.widget.add_css_class("devel");
        }
        self.paginator.add_page(WelcomePageWidget::new().widget);
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

        self.widget.set_content(Some(&self.paginator));
    }
}
