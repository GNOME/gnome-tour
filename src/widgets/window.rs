use adw::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use super::paginator::PaginatorWidget;
use crate::Application;

mod imp {
    use super::*;
    use crate::config;
    use crate::widgets::ImagePageWidget;
    use adw::subclass::prelude::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/Tour/ui/window.ui")]
    pub struct Window {
        #[template_child]
        pub(super) paginator: TemplateChild<PaginatorWidget>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            ImagePageWidget::static_type();
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self, widget: &Self::Type) {
            widget.set_icon_name(Some(config::APP_ID));

            // Devel Profile
            if config::PROFILE == "Devel" {
                widget.add_css_class("devel");
            }
            self.parent_constructed(widget);
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
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        glib::Object::new(&[("application", app)]).unwrap()
    }

    pub fn paginator(&self) -> PaginatorWidget {
        self.imp().paginator.clone()
    }

    pub fn start_tour(&self) {
        self.imp().paginator.set_page(1);
    }

    pub fn reset_tour(&self) {
        self.imp().paginator.set_page(0);
    }
}
