use crate::config;
use crate::utils;
use crate::widgets::Window;
use adw::prelude::*;
use gtk::{
    gio,
    glib::{self, clone},
    subclass::prelude::*,
};
use log::info;

mod imp {
    use super::*;
    use adw::subclass::prelude::*;
    use gtk::glib::{once_cell::sync::OnceCell, WeakRef};

    #[derive(Debug, Default)]
    pub struct Application {
        pub(super) window: OnceCell<WeakRef<Window>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type ParentType = adw::Application;
        type Type = super::Application;
    }

    impl ObjectImpl for Application {}
    impl ApplicationImpl for Application {
        fn activate(&self, application: &Self::Type) {
            let window = Window::new(&application);
            application.add_window(&window);
            window.present();
            self.window.set(window.downgrade()).unwrap();
            self.parent_activate(application);
        }

        fn startup(&self, application: &Self::Type) {
            // Quit
            utils::action(
                application,
                "quit",
                clone!(@weak application => move |_, _| {
                    application.quit();
                }),
            );

            // Start Tour
            utils::action(
                application,
                "start-tour",
                clone!(@weak application => move |_, _| {
                    application.window().start_tour();
                }),
            );

            // Skip Tour
            utils::action(
                application,
                "skip-tour",
                clone!(@weak application => move |_, _| {
                    application.quit();
                }),
            );

            utils::action(
                application,
                "next-page",
                clone!(@weak application => move |_, _| {
                    let window = application.window();
                    if window.paginator().try_next().is_none() {
                        window.close();
                    }
                }),
            );

            utils::action(
                application,
                "previous-page",
                clone!(@weak application => move |_, _| {
                    let window = application.window();
                    if window.paginator().try_previous().is_none() {
                        window.reset_tour();
                    }
                }),
            );
            application.set_accels_for_action("app.quit", &["<Control>q"]);
            self.parent_startup(application);
        }
    }
    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &config::APP_ID),
            ("resource-base-path", &Some("/org/gnome/Tour")),
        ])
        .unwrap()
    }

    fn window(&self) -> Window {
        self.imp().window.get().and_then(|w| w.upgrade()).unwrap()
    }

    pub fn run() {
        info!("GNOME Tour ({})", config::APP_ID);
        info!("Version: {} ({})", config::VERSION, config::PROFILE);
        info!("Datadir: {}", config::PKGDATADIR);
        let app = Self::new();
        gtk::prelude::ApplicationExtManual::run(&app);
    }
}
