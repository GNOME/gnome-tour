// based on https://gitlab.gnome.org/World/podcasts/-/blob/master/podcasts-gtk/src/i18n|utils.rs
use gettextrs::gettext;

pub fn action<T, F>(thing: &T, name: &str, action: F)
where
    T: gio::ActionMapExt,
    for<'r, 's> F: Fn(&'r gio::SimpleAction, Option<&glib::Variant>) + 'static,
{
    // Create a stateless, parameterless action
    let act = gio::SimpleAction::new(name, None);
    // Connect the handler
    act.connect_activate(action);
    // Add it to the map
    thing.add_action(&act);
}

pub fn i18n_f(format: &str, args: &[&str]) -> String {
    let s = gettext(format);
    let mut parts = s.split("{}");
    let mut output = parts.next().unwrap_or_default().to_string();
    for (p, a) in parts.zip(args.iter()) {
        output += &(a.to_string() + &p.to_string());
    }
    output
}
