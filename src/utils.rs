// based on https://gitlab.gnome.org/World/podcasts/-/blob/master/podcasts-gtk/src/i18n|utils.rs
use gettextrs::gettext;
use gtk::{gio, glib};
use regex::{Captures, Regex};

pub fn action<T, F>(thing: &T, name: &str, action: F)
where
    T: gio::traits::ActionMapExt,
    for<'r, 's> F: Fn(&'r gio::SimpleAction, Option<&glib::Variant>) + 'static,
{
    // Create a stateless, parameterless action
    let act = gio::SimpleAction::new(name, None);
    // Connect the handler
    act.connect_activate(action);
    // Add it to the map
    thing.add_action(&act);
}

pub fn i18n_f(format: &str, kwargs: &[(&str, &str)]) -> String {
    let mut s = gettext(format);
    for (k, v) in kwargs {
        if let Ok(re) = Regex::new(&format!("\\{{{}\\}}", k)) {
            s = re
                .replace_all(&s, |_: &Captures<'_>| v.to_string())
                .to_string();
        }
    }
    s
}
