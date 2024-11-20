use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

use crate::{config, widgets::Window};

mod imp {
    use std::cell::OnceCell;

    use glib::WeakRef;

    use super::*;

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

            if let Some(window) = application.active_window() {
                window.present();
                return;
            }

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
            application.add_action_entries([quit]);

            application.set_accels_for_action("app.quit", &["<Control>q"]);
            application.set_accels_for_action("win.skip-tour", &["Escape"]);
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
