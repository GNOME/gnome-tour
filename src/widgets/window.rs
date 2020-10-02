use crate::utils::i18n_f;
use gettextrs::gettext;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::pages::{ImagePageWidget, WelcomePageWidget};
use super::paginator::PaginatorWidget;
use crate::config::{APP_ID, PROFILE};

pub struct Window {
    pub widget: libhandy::ApplicationWindow,
    pub paginator: RefCell<Rc<PaginatorWidget>>,
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        let widget = libhandy::ApplicationWindow::new();
        widget.set_application(Some(app));

        let paginator = RefCell::new(PaginatorWidget::new());

        let mut window_widget = Window { widget, paginator };

        window_widget.init();
        window_widget
    }

    pub fn start_tour(&self) {
        self.paginator.borrow_mut().set_page(1);
    }

    pub fn reset_tour(&self) {
        self.paginator.borrow_mut().set_page(0);
    }

    fn init(&mut self) {
        self.widget.set_default_size(960, 720);
        self.widget.set_icon_name(Some(APP_ID));

        // Devel Profile
        if PROFILE == "Devel" {
            self.widget.get_style_context().add_class("devel");
        }
        self.paginator
            .borrow_mut()
            .add_page(WelcomePageWidget::new().widget.upcast::<gtk::Widget>());

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/activities.svg",
                gettext("Some things have changed since the last version"),
                gettext("Easier application launching and window switching with the new overview layout. \nSee the new overview in the Activities corner."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/favorites.svg",
                gettext("Favourites are at the bottom"),
                gettext("To see more apps, swipe up or click the dot grid icon."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/workspaces.svg",
                gettext("Workspaces are easier"),
                gettext("To move between groups of windows, swipe or scroll or click next workspace."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/search.svg",
                gettext("Search is still king"),
                gettext("In the activities view, just start typing to search for apps, settings and more."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/touch.svg",
                gettext("Touchpad gestures"),
                gettext("The new arrangement comes with new touchpad gestures. Use four finger drags to move between workspaces, \nopen and close overview and view the app grid."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        let name = glib::get_os_info("NAME").unwrap_or_else(|| "GNOME".into());
        let last_page = ImagePageWidget::new(
            "/org/gnome/Tour/ready-to-go.svg",
            // Translators: The following string is formated as "We hope that you enjoy GNOME"
            i18n_f("That's it! We hope that you enjoy {}.", &[&name]),
            gettext("To get more advice and tips, see the Help app."),
        );
        last_page.widget.get_style_context().add_class("last-page");
        self.paginator
            .borrow_mut()
            .add_page(last_page.widget.upcast::<gtk::Widget>());

        self.widget.add(&self.paginator.borrow().widget);
    }
}
