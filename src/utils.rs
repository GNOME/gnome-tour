// based on https://gitlab.gnome.org/World/podcasts/-/blob/master/podcasts-gtk/src/i18n|utils.rs
use gettextrs::gettext;
use regex::{Captures, Regex};

pub fn i18n_f(format: &str, kwargs: &[(&str, &str)]) -> String {
    let mut s = gettext(format);
    for (k, v) in kwargs {
        if let Ok(re) = Regex::new(&format!("\\{{{k}\\}}")) {
            s = re
                .replace_all(&s, |_: &Captures<'_>| v.to_string())
                .to_string();
        }
    }
    s
}
