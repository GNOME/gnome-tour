#[cfg(feature = "video")]
use crate::config;
use gettextrs::gettext;
#[cfg(feature = "video")]
use gio::FileExt;
use gtk::prelude::*;

pub struct WelcomePageWidget {
    pub widget: libhandy::WindowHandle,
}

impl WelcomePageWidget {
    pub fn new() -> Self {
        let widget = libhandy::WindowHandle::new();
        let welcome_page = Self { widget };

        welcome_page.init();
        welcome_page
    }

    #[cfg(not(feature = "video"))]
    fn get_header_widget(&self) -> gtk::Widget {
        let icon = glib::get_os_info("LOGO").unwrap_or_else(|| "start-here-symbolic".into());

        let logo = gtk::Image::from_icon_name(Some(&icon), gtk::IconSize::Dialog);
        logo.set_pixel_size(196);
        logo.show();

        logo.upcast::<gtk::Widget>()
    }

    #[cfg(feature = "video")]
    fn get_header_widget(&self) -> gtk::Widget {
        let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
        let sink = gst::ElementFactory::make("gtksink", None).expect("Missing dependency: element gtksink is needed (usually, in gstreamer-plugins-good or in gst-plugin-gtk).");
        let renderer = gst_player::PlayerVideoOverlayVideoRenderer::with_sink(&sink).upcast();
        let player = gst_player::Player::new(Some(&renderer), Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()));

        let video_file = gio::File::new_for_path(config::VIDEO_PATH);
        player.set_uri(&video_file.get_uri());

        let video_widget = player
            .get_pipeline()
            .get_property("video-sink")
            .unwrap()
            .get::<gst::Element>()
            .expect("The player of a VideoPlayerWidget should not use the default sink.")
            .unwrap()
            .get_property("widget")
            .unwrap()
            .get::<gtk::Widget>()
            .unwrap()
            .unwrap();

        video_widget.set_size_request(-1, 300);
        video_widget.set_property("ignore-alpha", &false).unwrap();
        video_widget.show();

        gtk::idle_add(clone!(@strong player => move || {
            player.play();
            glib::Continue(true)
        }));

        video_widget
    }

    fn init(&self) {
        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        container.set_property_expand(true);
        container.set_valign(gtk::Align::Center);
        container.set_halign(gtk::Align::Center);
        container.set_margin_top(24);
        container.set_margin_bottom(24);

        let name = glib::get_os_info("NAME").unwrap_or_else(|| "GNOME".into());
        let version = glib::get_os_info("VERSION").unwrap_or_else(|| "3.36".into());

        let header = self.get_header_widget();
        container.add(&header);

        let title = gtk::Label::new(Some(&gettext(format!("Welcome to {} {}", name, version))));
        title.set_margin_top(36);
        title.get_style_context().add_class("large-title");
        title.show();
        container.add(&title);

        let text = gtk::Label::new(Some(&gettext("Hi there! Take the tour to learn your way around and discover essential features.")));
        text.get_style_context().add_class("body");
        text.set_margin_top(12);
        text.show();
        container.add(&text);

        let actions_container = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        actions_container.set_halign(gtk::Align::Center);
        actions_container.set_margin_top(36);

        let skip_tour_btn = gtk::Button::with_label(&gettext("_No Thanks"));
        skip_tour_btn.set_property_height_request(40);
        skip_tour_btn.set_property_width_request(180);
        skip_tour_btn.set_use_underline(true);
        skip_tour_btn.set_action_name(Some("app.skip-tour"));
        skip_tour_btn.show();
        actions_container.add(&skip_tour_btn);

        let start_tour_btn = gtk::Button::with_label(&gettext("_Start Tour"));
        start_tour_btn.set_property_height_request(40);
        start_tour_btn.set_property_width_request(180);
        start_tour_btn.set_use_underline(true);
        start_tour_btn.set_action_name(Some("app.start-tour"));
        start_tour_btn.get_style_context().add_class("suggested-action");
        start_tour_btn.show();
        actions_container.add(&start_tour_btn);
        actions_container.set_focus_child(Some(&start_tour_btn));

        actions_container.show();

        container.add(&actions_container);

        container.show();
        self.widget.add(&container);
        self.widget.show();
    }
}
