use adw::{prelude::*, subclass::prelude::*};
use gtk::{gio, glib};

use super::paginator::PaginatorWidget;
use crate::{utils::i18n_f, Application};

mod imp {
    use super::*;
    use crate::{config, widgets::ImagePageWidget};

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/Tour/ui/window.ui")]
    pub struct Window {
        #[template_child]
        pub(super) paginator: TemplateChild<PaginatorWidget>,
        #[template_child]
        pub(super) welcome_page: TemplateChild<ImagePageWidget>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            // Start Tour
            klass.install_action("win.start-tour", None, |win, _, _| win.start_tour());
            // Skip Tour
            klass.install_action("win.skip-tour", None, |win, _, _| {
                win.application().unwrap().quit();
            });
            // Next page
            klass.install_action("win.next-page", None, |win, _, _| {
                if win.imp().paginator.try_next().is_none() {
                    win.close();
                }
            });
            // Previous page
            klass.install_action("win.previous-page", None, |win, _, _| {
                if win.imp().paginator.try_previous().is_none() {
                    win.reset_tour();
                }
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();
            let widget = self.obj();
            widget.set_icon_name(Some(config::APP_ID));

            // Devel Profile
            if config::PROFILE == "Devel" {
                widget.add_css_class("devel");
            }
            let name = glib::os_info("NAME").unwrap_or_else(|| "GNOME".into());
            let version = glib::os_info("VERSION").unwrap_or_else(|| "".into());

            let body = i18n_f(
                // Translators: The following string is formatted as "Learn about new and essential
                // features in GNOME 3.36" for example
                "Learn about the key features in {name} {version}.",
                &[("name", &name), ("version", &version)],
            );
            self.welcome_page.set_body(body);
        }
    }
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Accessible,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root,
                    gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn start_tour(&self) {
        self.imp().paginator.set_page(1);
    }

    pub fn reset_tour(&self) {
        self.imp().paginator.set_page(0);
    }
}
