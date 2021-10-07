#[cfg(feature = "video")]
use crate::config;
use crate::utils::i18n_f;
use gettextrs::gettext;
#[cfg(feature = "video")]
use gio::FileExt;
use gtk::glib;
#[cfg(feature = "video")]
use gtk::glib::clone;
#[cfg(feature = "video")]
use gtk::glib::{Receiver, Sender};
use gtk::prelude::*;
#[cfg(feature = "video")]
use std::cell::RefCell;

#[derive(PartialEq)]
#[cfg(feature = "video")]
pub enum Action {
    VideoReady,
    VideoUp,
}

pub struct WelcomePageWidget {
    pub widget: gtk::Box,
    #[cfg(feature = "video")]
    player: gst_player::Player,
    #[cfg(feature = "video")]
    receiver: RefCell<Option<Receiver<Action>>>,
    #[cfg(feature = "video")]
    sender: Sender<Action>,
}

impl WelcomePageWidget {
    pub fn new() -> Self {
        let widget = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        #[cfg(feature = "video")]
        let player = {
            let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
            let sink = gst::ElementFactory::make("gtksink", None)
                .expect("Missing dependency: element gtksink is needed (usually, in gstreamer-plugins-good or in gst-plugin-gtk).");
            let renderer = gst_player::PlayerVideoOverlayVideoRenderer::with_sink(&sink).upcast();
            gst_player::Player::new(
                Some(&renderer),
                Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
            )
        };
        #[cfg(feature = "video")]
        let (sender, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        #[cfg(feature = "video")]
        let receiver = RefCell::new(Some(r));

        let welcome_page = Self {
            widget,
            #[cfg(feature = "video")]
            player,
            #[cfg(feature = "video")]
            sender,
            #[cfg(feature = "video")]
            receiver,
        };

        welcome_page.init();
        welcome_page
    }

    fn init(&self) {
        let container = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .hexpand(true)
            .vexpand(true)
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .margin_top(24)
            .margin_bottom(24)
            .build();
        self.widget.add_css_class("page");
        self.widget.add_css_class("welcome-page");

        let clamp = libadwaita::Clamp::new();
        clamp.set_child(Some(&container));

        #[cfg(not(feature = "video"))]
        let header = {
            let logo = gtk::PictureBuilder::new()
                .can_shrink(false)
                .keep_aspect_ratio(true)
                .build();
            logo.set_resource(Some("/org/gnome/Tour/welcome.svg"));

            logo.upcast::<gtk::Widget>()
        };

        #[cfg(feature = "video")]
        let header = {
            let video_widget = self
                .player
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

            video_widget.set_size_request(-1, 360);
            video_widget.set_property("ignore-alpha", &false).unwrap();
            video_widget.show();
            video_widget.add_css_class("video");
            video_widget
        };

        container.append(&header);

        #[cfg(feature = "video")]
        {
            let receiver = self.receiver.borrow_mut().take().unwrap();
            receiver.attach(
                None,
                clone!(@strong self.player as player => move |action| {
                    match action {
                        Action::VideoReady => player.play(),
                        Action::VideoUp => header.add_css_class("playing"),
                    };
                    glib::Continue(true)
                }),
            );

            self.player.connect_state_changed(
                clone!(@strong self.sender as sender => move |_p,state| {
                    if state == gst_player::PlayerState::Playing {
                        sender.send(Action::VideoUp).unwrap();
                    }
                }),
            );

            self.player.connect_uri_loaded(
                clone!(@strong self.sender as sender => move |_p, _uri| {
                    sender.send(Action::VideoReady).unwrap();
                }),
            );
            self.player.connect_end_of_stream(move |p| p.stop());

            let video_file = gio::File::new_for_path(config::VIDEO_PATH);
            gtk::timeout_add(
                500,
                clone!(@strong self.player as player => move || {
                    player.set_uri(&video_file.get_uri());
                    glib::Continue(false)
                }),
            );
        };

        let title = gtk::Label::new(Some(&gettext("Start the Tour")));
        title.set_margin_top(36);
        title.add_css_class("page-title");
        container.append(&title);

        let name = glib::os_info("NAME").unwrap_or_else(|| "GNOME".into());
        let version = glib::os_info("VERSION").unwrap_or_else(|| "".into());
        // Translators: The following string is formated as "Learn about new and essential features in GNOME 3.36" for example
        let text = gtk::Label::new(Some(&i18n_f(
            "Learn about the key features in {} {}.",
            &[&name, &version],
        )));
        text.add_css_class("body");
        text.set_margin_top(12);
        container.append(&text);

        self.widget.append(&clamp);
    }
}
