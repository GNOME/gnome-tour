use gtk::prelude::*;

pub struct ImagePageWidget {
    pub widget: gtk::Box,
}

impl ImagePageWidget {
    pub fn new(resource_uri: &str, head: String, body: String) -> Self {
        let widget = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let image_page = Self { widget };

        image_page.init(resource_uri, head, body);
        image_page
    }

    fn init(&self, resource_uri: &str, head: String, body: String) {
        self.widget.set_hexpand(true);
        self.widget.set_vexpand(true);
        self.widget.add_css_class("page");
        self.widget.set_halign(gtk::Align::Fill);
        self.widget.set_valign(gtk::Align::Fill);

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
        let clamp = libadwaita::Clamp::new();
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
        head_label.add_css_class("page-title");
        container.append(&head_label);

        let body_label = gtk::Label::builder()
            .label(&body)
            .lines(2)
            .wrap(true)
            .justify(gtk::Justification::Center)
            .valign(gtk::Align::Center)
            .margin_top(12)
            .build();
        body_label.add_css_class("page-body");
        container.append(&body_label);

        self.widget.append(&clamp);
    }
}
