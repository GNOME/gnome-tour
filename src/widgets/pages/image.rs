use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct ImagePageWidget;

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
            self.parent_constructed(obj);
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
        ])
        .unwrap();
        image_page.init(resource_uri, head, body);
        image_page
    }

    fn init(&self, resource_uri: &str, head: String, body: String) {
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

        let picture = gtk::Picture::builder()
            .can_shrink(false)
            .keep_aspect_ratio(true)
            .build();
        picture.set_resource(Some(resource_uri));
        container.append(&picture);

        let head_label = gtk::Label::builder()
            .label(&head)
            .justify(gtk::Justification::Center)
            .valign(gtk::Align::Center)
            .margin_top(36)
            .build();
        head_label.add_css_class("title-1");
        container.append(&head_label);

        let body_label = gtk::Label::builder()
            .label(&body)
            .lines(2)
            .wrap(true)
            .justify(gtk::Justification::Center)
            .valign(gtk::Align::Center)
            .margin_top(12)
            .build();
        container.append(&body_label);

        self.append(&clamp);
    }
}
