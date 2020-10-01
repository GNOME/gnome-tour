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
        self.widget.set_property_expand(true);
        self.widget.set_halign(gtk::Align::Fill);
        self.widget.set_valign(gtk::Align::Fill);

        let container = gtk::BoxBuilder::new()
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

        let image = gtk::Image::from_resource(&resource_uri);
        image.set_valign(gtk::Align::Start);
        image.show();
        container.add(&image);

        let head_label = gtk::LabelBuilder::new()
            .label(&head)
            .justify(gtk::Justification::Center)
            .valign(gtk::Align::Center)
            .margin_top(36)
            .build();
        head_label.get_style_context().add_class("page-title");
        head_label.show();
        container.add(&head_label);

        let body_label = gtk::LabelBuilder::new()
            .label(&body)
            .lines(2)
            .wrap(true)
            .justify(gtk::Justification::Center)
            .valign(gtk::Align::Center)
            .margin_top(12)
            .build();
        body_label.get_style_context().add_class("page-body");
        body_label.show();
        container.add(&body_label);

        container.show();
        self.widget.add(&container);
        self.widget.show();
    }
}
