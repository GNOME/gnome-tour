use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use glib::{ParamSpec, Properties, Value};
    use gtk::glib::once_cell::sync::OnceCell;
    use std::cell::RefCell;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::ImagePageWidget)]
    pub struct ImagePageWidget {
        #[property(get, set= Self::set_resource_uri, construct_only)]
        pub(super) resource_uri: OnceCell<String>,
        #[property(get, set, construct_only)]
        pub(super) head: OnceCell<String>,
        #[property(get, set, construct)]
        pub(super) body: RefCell<Option<String>>,
        pub(super) picture: gtk::Picture,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImagePageWidget {
        const NAME: &'static str = "ImagePageWidget";
        type ParentType = gtk::Box;
        type Type = super::ImagePageWidget;
    }

    impl ObjectImpl for ImagePageWidget {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            let layout_manager = obj
                .layout_manager()
                .map(|l| l.downcast::<gtk::BoxLayout>().unwrap())
                .unwrap();
            layout_manager.set_orientation(gtk::Orientation::Vertical);
            obj.add_css_class("page");

            obj.set_hexpand(true);
            obj.set_vexpand(true);
            obj.set_halign(gtk::Align::Fill);
            obj.set_valign(gtk::Align::Fill);

            let container = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(12)
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .vexpand(true)
                .margin_bottom(48)
                .margin_top(12)
                .margin_start(12)
                .margin_end(12)
                .build();
            let clamp = adw::Clamp::new();
            clamp.set_child(Some(&container));

            self.picture.set_can_shrink(false);
            self.picture.set_content_fit(gtk::ContentFit::Contain);
            container.append(&self.picture);

            let head_label = gtk::Label::builder()
                .justify(gtk::Justification::Center)
                .valign(gtk::Align::Center)
                .margin_top(36)
                .build();
            obj.bind_property("head", &head_label, "label")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();

            head_label.add_css_class("title-1");
            container.append(&head_label);

            let body_label = gtk::Label::builder()
                .lines(2)
                .wrap(true)
                .justify(gtk::Justification::Center)
                .valign(gtk::Align::Center)
                .margin_top(12)
                .build();
            obj.bind_property("body", &body_label, "label")
                .flags(glib::BindingFlags::SYNC_CREATE)
                .build();
            container.append(&body_label);

            obj.append(&clamp);
        }

        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
    impl WidgetImpl for ImagePageWidget {}
    impl BoxImpl for ImagePageWidget {}

    impl ImagePageWidget {
        fn set_resource_uri(&self, resource_uri: &str) {
            self.picture.set_resource(Some(resource_uri));
            self.resource_uri.set(resource_uri.to_owned()).unwrap();
        }
    }
}

glib::wrapper! {
    pub struct ImagePageWidget(ObjectSubclass<imp::ImagePageWidget>)
        @extends gtk::Widget, gtk::Box;
}

impl ImagePageWidget {
    pub fn new(resource_uri: &str, head: String, body: String) -> Self {
        glib::Object::builder()
            .property("resource-uri", &resource_uri)
            .property("head", &head)
            .property("body", &body)
            .build()
    }
}
