use crate::config;
use crate::widgets::Window;
use adw::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};

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
        fn activate(&self) {
            self.parent_activate();
            let application = self.obj();

            let window = Window::new(&application);
            application.add_window(&window);
            window.present();
            self.window.set(window.downgrade()).unwrap();
        }

        fn startup(&self) {
            self.parent_startup();
            let application = self.obj();
            // Quit
            let quit = gio::ActionEntry::builder("quit")
                .activate(move |app: &Self::Type, _, _| app.quit())
                .build();
            // Start Tour
            let start_tour = gio::ActionEntry::builder("start-tour")
                .activate(move |app: &Self::Type, _, _| app.window().start_tour())
                .build();
            // Skip Tour
            let skip_tour = gio::ActionEntry::builder("skip-tour")
                .activate(move |app: &Self::Type, _, _| app.quit())
                .build();
            // Next page
            let next_page = gio::ActionEntry::builder("next-page")
                .activate(move |app: &Self::Type, _, _| {
                    let window = app.window();
                    if window.paginator().try_next().is_none() {
                        window.close();
                    }
                })
                .build();
            // Previous page
            let previous_page = gio::ActionEntry::builder("previous-page")
                .activate(move |app: &Self::Type, _, _| {
                    let window = app.window();
                    if window.paginator().try_previous().is_none() {
                        window.reset_tour();
                    }
                })
                .build();
            application.add_action_entries([quit, start_tour, skip_tour, next_page, previous_page]);

            application.set_accels_for_action("app.quit", &["<Control>q"]);
            application.set_accels_for_action("app.skip-tour", &["Escape"]);
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
    fn window(&self) -> Window {
        self.imp().window.get().and_then(|w| w.upgrade()).unwrap()
    }

    pub fn run() -> glib::ExitCode {
        log::info!("GNOME Tour ({})", config::APP_ID);
        log::info!("Version: {} ({})", config::VERSION, config::PROFILE);
        log::info!("Datadir: {}", config::PKGDATADIR);
        Self::default().run()
    }
}

impl Default for Application {
    fn default() -> Self {
        glib::Object::builder()
            .property("application-id", config::APP_ID)
            .property("resource-base-path", "/org/gnome/Tour")
            .build()
    }
}
