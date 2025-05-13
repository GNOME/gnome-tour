use gettextrs::*;
use gtk::{gio, glib};

mod application;
mod config;
mod utils;
mod widgets;

use application::Application;
use config::{GETTEXT_PACKAGE, LOCALEDIR};

fn main() -> glib::ExitCode {
    let mut log_builder = env_logger::builder();
    // Compatibility G_MESSAGES_DEBUG env var
    if !glib::log_writer_default_would_drop(glib::LogLevel::Debug, Some("gnome_tour")) {
        log_builder.filter_module("gnome_tour", log::LevelFilter::Debug);
    }
    log_builder.init();

    // Prepare i18n
    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR)
        .unwrap_or_else(|_| panic!("Unable to bind text domain for {GETTEXT_PACKAGE}"));
    textdomain(GETTEXT_PACKAGE)
        .unwrap_or_else(|_| panic!("Unable to switch to text domain {GETTEXT_PACKAGE}"));

    glib::set_application_name(&gettext("Tour"));

    let res = gio::Resource::load(config::RESOURCES_FILE).expect("Could not load resources");
    gio::resources_register(&res);

    Application::run()
}
