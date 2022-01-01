use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;
    use glib::once_cell::sync::Lazy;
    use glib::{ParamFlags, ParamSpec, ParamSpecString, Value};
    use gtk::glib::once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct ImagePageWidget {
        pub(super) resource_uri: OnceCell<String>,
        pub(super) head: OnceCell<String>,
        pub(super) body: OnceCell<String>,
        pub(super) picture: gtk::Picture,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImagePageWidget {
        const NAME: &'static str = "ImagePageWidget";
        type ParentType = gtk::Box;
        type Type = super::ImagePageWidget;
    }

    impl ObjectImpl for ImagePageWidget {
        fn constructed(&self, obj: &Self::Type) {
            let layout_manager = obj
                .layout_manager()
                .map(|l| l.downcast::<gtk::BoxLayout>().unwrap())
                .unwrap();
            layout_manager.set_orientation(gtk::Orientation::Vertical);
            obj.add_css_class("page");

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
            self.picture.set_keep_aspect_ratio(true);
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
            self.parent_constructed(obj);
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(
                        "resource-uri",
                        "Resource URI",
                        "Resource URI of the image",
                        None,
                        ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                    ),
                    ParamSpecString::new(
                        "head",
                        "Head",
                        "The title of the page",
                        None,
                        ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                    ),
                    ParamSpecString::new(
                        "body",
                        "Body",
                        "The body of the page",
                        None,
                        ParamFlags::READWRITE | ParamFlags::CONSTRUCT_ONLY,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "resource-uri" => {
                    let resource_uri: String = value.get().unwrap();
                    self.picture.set_resource(Some(&resource_uri));
                    self.resource_uri.set(resource_uri).unwrap();
                }
                "head" => {
                    let head = value.get().unwrap();
                    self.head.set(head).unwrap();
                }
                "body" => {
                    let body = value.get().unwrap();
                    self.body.set(body).unwrap();
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "resource-uri" => self.resource_uri.get().to_value(),
                "head" => self.head.get().to_value(),
                "body" => self.body.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for ImagePageWidget {}
    impl BoxImpl for ImagePageWidget {}
}

glib::wrapper! {
    pub struct ImagePageWidget(ObjectSubclass<imp::ImagePageWidget>)
        @extends gtk::Widget, gtk::Box;
}

impl ImagePageWidget {
    pub fn new(resource_uri: &str, head: String, body: String) -> Self {
        let image_page = glib::Object::new::<Self>(&[
            ("hexpand", &true),
            ("vexpand", &true),
            ("halign", &gtk::Align::Fill),
            ("valign", &gtk::Align::Fill),
            ("resource-uri", &resource_uri),
            ("head", &head),
            ("body", &body),
        ])
        .unwrap();
        image_page
    }
}
