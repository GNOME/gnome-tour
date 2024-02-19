use gettextrs::*;
use gtk::{gio, glib};

mod application;
mod config;
mod utils;
mod widgets;

use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() -> glib::ExitCode {
    env_logger::init();
    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
        .unwrap_or_else(|_| panic!("Unable to bind text domain for {GETTEXT_PACKAGE}"));
    textdomain(GETTEXT_PACKAGE)
        .unwrap_or_else(|_| panic!("Unable to switch to text domain {GETTEXT_PACKAGE}"));

    glib::set_application_name(&gettext("Tour"));
    glib::set_prgname(Some("Tour"));

    let res = gio::Resource::load(config::RESOURCES_FILE).expect("Could not load resources");
    gio::resources_register(&res);

    Application::run()
}
