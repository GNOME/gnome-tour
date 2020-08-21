use gettextrs::*;

mod application;
mod config;
mod static_resources;
mod utils;
mod widgets;

use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() {
    pretty_env_logger::init();
    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR);
    textdomain(GETTEXT_PACKAGE);

    glib::set_application_name("Tour");
    glib::set_prgname(Some("Tour"));

    gtk::init().expect("Unable to start GTK3");
    #[cfg(feature = "video")]
    gst::init().expect("Unable to start gst");

    static_resources::init().expect("Failed to initialize the resource file.");

    let app = Application::new();
    app.run();
}
