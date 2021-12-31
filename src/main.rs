use gettextrs::*;
use gtk::glib;

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
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
        .unwrap_or_else(|_| panic!("Unable to bind text domain for {}", GETTEXT_PACKAGE));
    textdomain(GETTEXT_PACKAGE)
        .unwrap_or_else(|_| panic!("Unable to switch to text domain {}", GETTEXT_PACKAGE));

    glib::set_application_name(&gettext("Tour"));
    glib::set_prgname(Some("Tour"));

    gtk::init().expect("Unable to start GTK3");
    #[cfg(feature = "video")]
    gst::init().expect("Unable to start gst");

    static_resources::init().expect("Failed to initialize the resource file.");

    Application::run()
}
